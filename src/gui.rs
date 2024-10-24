use guess_word::*;
use iced::alignment::{Alignment, Horizontal};
use iced::text_input::TextInput;
use iced::{Application, Column, Command, Element, Length, Row, Text};

#[derive(Debug, Clone)]
struct GuessResultWidget {
    letters: Vec<GuessLetter>,
}

impl GuessResultWidget {
    fn new() -> Self {
        Self {
            // ジェネリックでこういう書き方できるのか??
            letters: Vec::<GuessLetter>::with_capacity(GUESS_LENGTH),
        }
    }

    fn view(&mut self) -> Element<Message> {
        // クロージャもこういう書き方できるのか...
        // 共通化している感じ
        let new_text = |label, size, color| {
            Text::new(label)
                .width(Length::Fill)
                .size(size)
                .color(color)
                .horizontal_alignment(Horizontal::Center)
        };

        self.letters
            .iter_mut() // 各値の可変参照を取得する
            .fold(
                Row::new().spacing(20).align_items(Alignment::Center),
                |row, l| {
                    row.push(match l.accuracy {
                        HitAccuracy::InRightPlace => new_text(
                            l.letter.to_string(), 30, [1.0, 0.0, 0.0]
                        ),
                        HitAccuracy::InWord => new_text(
                            l.letter.to_string(), 30, [1.0, 0.5, 0.0]
                        ),
                        HitAccuracy::NotInWord => new_text(
                            l.letter.to_string(), 30, [0.9; 3]
                        ),
                    })
                }
            )
            .into()
    }
}

#[derive(Debug, Default)]
struct State {
    input: iced::text_input::State,
    input_value: String,
    announce: String,
    guesses: Vec<GuessResultWidget>,
}

#[derive(Debug, Clone)]
pub enum Message { // 他でいう Event のようなものを定義するのかな
    InputChanged(String),
    Guess,
}

#[derive(Default)]
pub struct GuessWord {
    game: Game,
    state: State,
}

impl GuessWord {
    fn create_guess_result_widget(&mut self) {
        let word = self.game.guesses().last().unwrap();
        let mut widget = GuessResultWidget::new(); // 可変で束縛
        widget.letters = word.letters().iter().map(|l| l.clone()).collect();
        self.state.guesses.push(widget);
    }
}

impl Application for GuessWord {
    // TODO: `type` について詳しく調べる必要がありそう
    // 「関連型」 たしかコレクションとかにおける Item みたいなやつ
    type Executor = iced::executor::Default; // 形に別名をつける...
    type Message = Message; // enum のMessage を Application の関連型Messageとして設定
    type Flags = ();

    // 見たことのない文法が多い
    // ユニット型の引数 `_flags` を受け取る
    // GuessWord と ジェネリックでこのMessage（ユニットの別名）が指定されたCommand をタプルで返す
    fn new(_flags: ()) -> (GuessWord, Command<Self::Message>) {
        (GuessWord::default(), Command::none())
    }

    // タイトルの設定
    fn title(&self) -> String {
        "GuessWord".to_string()
    }

    // メッセージを受け取って更新する
    // Command はバックグランドで実行されれる非同期タスク、本来はMessageを返す
    fn update(&mut self, message: Self::Message) -> Command<Self::Message> {
        match message {
            // InputChange のメッセージを処理 どこでdispachするんだろう
            Message::InputChanged(value) => {
                // 所有権を渡している...
                self.state.input_value = value;
            },
            Message::Guess => {
                // 状態に持っている入力値の不変参照を渡す
                let old_status= self.game.game_status();
                let (status, result) = self.game.guess(&self.state.input_value);
                match status {
                    GameStatus::Won => {
                        if old_status != status {
                            self.create_guess_result_widget();
                        }
                        self.state.announce = "You Win!".to_string();
                    },
                    GameStatus::Lost => {
                        if old_status != status {
                            self.create_guess_result_widget();
                        }
                        self.state.announce = format!("You Lost! (answer: {})",
                            self.game.get_answer().unwrap());
                    },
                    GameStatus::InProgress => match result {
                        GuessResult::DuplicateGuess => {
                            self.state.announce = "Duplicate Guess".to_string();
                        },
                        GuessResult::IncorrectLength => {
                            self.state.announce = "Incorrect Length".to_string();
                        },
                        GuessResult::NotInDictionary => {
                            self.state.announce = "Invalid word".to_string();
                        },
                        GuessResult::Valid => {
                            self.create_guess_result_widget();
                            self.state.announce.clear();
                        }
                        _ => ()
                    }
                }
                self.state.input_value.clear();
            }
        }
        Command::none()
    }

    // 画面描画
    fn view(&mut self) -> Element<Self::Message> {
        let title = Text::new("GuessWord")
            .width(Length::Fill)
            .size(60)
            .color([0.5; 3])
            .horizontal_alignment(Horizontal::Center);

        let announce = Text::new(&self.state.announce)
            .width(Length::Fill)
            .size(20)
            .color([0.0, 0.4, 1.0])
            .horizontal_alignment(Horizontal::Center);

        let input = TextInput::new(
            &mut self.state.input,
            "input word...",
            &mut self.state.input_value,
            Message::InputChanged
        )
        .padding(15)
        .size(30)
        .on_submit(Message::Guess);

        let results: Element<_> = self
            .state
            .guesses
            .iter_mut()
            .fold(Column::new().spacing(20), |column, guess| {
                column.push(guess.view())
            })
            .into();

        let content = Column::new()
            .padding(40)
            .max_width(800)
            .spacing(20)
            .push(title)
            .push(announce)
            .push(input)
            .push(results);
        content.into()
    }

}




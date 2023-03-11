use iced::{executor, Application, Command, Element, Settings};

struct GUI;

impl Application for GUI {
    type Executor = executor::Default;
    type Message = ();
    type Flags = ();
    type Theme = iced::Theme;

    // Application trate を run() した際にicedの内部で使われる初期化のためのメソッド
    fn new(_flags: ()) -> (GUI, Command<Self::Message>) {
        // 初期化時点で行いたい別処理をコマンドを使って実行できる
        // 今回は何もしないので none()
        (GUI, Command::none())
    }

    // ウィンドウのタイトルを設定
    fn title(&self) -> String {
        String::from("DEMO")
    }

    // ランタイムからメッセージを受け取った際に行う処理を記述
    fn update(&mut self, _message: Self::Message) -> Command<Self::Message> {
        Command::none()
    }

    // ウィンドウに表示する要素を記述するメソッド
    fn view(&self) -> Element<Self::Message> {
        Element::from("Hello, World!")
    }
}

fn main() {
    GUI::run(Settings::default());
}

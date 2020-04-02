pub mod robot;

struct ItemPos(u32, u32);

trait ItemEngine {
    fn select_item(&mut self, action: &Ror2Action) -> ItemPos;
}

enum Ror2Action {
    SelectWhite,
    SelectGreen,
    SelectRed,
    SelectLunar,
    SelectUseItem,
    SelectBossItem,
}

trait ActionDetector {
    fn read_screen(&mut self) -> Ror2Action;
}

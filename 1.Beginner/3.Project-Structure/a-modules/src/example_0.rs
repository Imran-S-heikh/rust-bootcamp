// Something's missing. Fix the code so that it compiles.

mod sausage_factory {
    // Don't let anybody outside of this module see this!
    fn get_secret_recipe() -> String {
        String::from("Ginger")
    }

    pub fn make_sausage() {
        get_secret_recipe();
        println!("sausage!");
    }
}

pub fn main() {
    sausage_factory::make_sausage();
}

// TODO: Fix the compiler error about calling a private function.
mod sausage_factory {
    // Don't let anybody outside of this module see this!
    fn get_secret_recipe() -> String {
        String::from("Ginger")
    }

    pub fn make_sausage() {
        get_secret_recipe();
        // this e above is invisible when we call in main
        println!("sausage!");
    }
    // it needs to be pub to be accessible outside of the module
}

fn main() {
    sausage_factory::make_sausage();
    // only prints "sausage!" because we're in the main function
}

// modules1.rs
// Execute `rustlings hint modules1` or use the `hint` watch subcommand for a hint.

mod sausage_factory {
    // Don't let anybody outside of this module see this!
    fn get_secret_recipe() -> String {
        String::from("Ginger")
    }

    pub fn make_sausage() {
        let secret_recipe = get_secret_recipe();
        println!("{}", format!("{secret_recipe} sausage!"));
    }
}

// main is same level as sausage_factory so no need for the mod to be public
// make_sausage needs to be public so it can be accessed by its parent,
// notice get_secret_recipe is private, this is because by default
// children have access to their ancestors
fn main() {
    sausage_factory::make_sausage();
}

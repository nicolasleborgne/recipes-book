
mod recipes_book {

    static mut RECIPES: Vec<&str> = Vec::new();

    pub fn table_of_content () -> &'static Vec<&'static str> {
        unsafe {
            &RECIPES
        }
    }

    pub fn read (title: &str) -> &str {
        unsafe {
            for recipe in &RECIPES {
                if recipe == &title {
                    return recipe
                }
            }
        }

        return ""
    }

    pub fn read_all () {

    }

    pub fn write (title: &'static str) {
        unsafe {
            RECIPES.push(title);
        }
    }

}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_list_recipes () {
        given_recipe("Poulet basquaise");

        assert_eq!(*recipes_book::table_of_content(), vec!["Poulet basquaise"]);
    }

    #[test]
    fn it_read_recipe () {
        given_recipe("Poulet basquaise");

        assert_eq!(recipes_book::read("Poulet basquaise"), "Poulet basquaise");
    }

    fn given_recipe(title: &'static str) {
        recipes_book::write(title);
    }
}
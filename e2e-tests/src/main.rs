mod tests {
    use headless_chrome::Browser;

    #[test]
    fn test_page_load() -> Result<(), failure::Error> {
        let browser = Browser::default()?;
        let tab = browser.wait_for_initial_tab()?;
        tab.navigate_to("http://localhost:4000")?;

        Ok(())
    }

    #[test]
    fn test_search_bar() -> Result<(), failure::Error> {
        let browser = Browser::default()?;
        let tab = browser.wait_for_initial_tab()?;
        tab.navigate_to("http://localhost:4000")?;

        // enter search query
        tab.wait_for_element(".mdc-text-field__input")?.click()?;
        tab.type_str("Porgy")?.press_key("Enter")?;
        let found_card_titles = tab.wait_for_elements(
            ".card:not(.filter--hidden) > .card__title-wrapper > .card__title",
        )?;
        assert!(found_card_titles.len() == 2);

        Ok(())
    }
}

fn main() {}

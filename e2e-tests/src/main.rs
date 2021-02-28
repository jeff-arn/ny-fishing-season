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
        tab.wait_for_element("#fish-text-field-input")?.click()?;
        tab.type_str("Porgy")?.press_key("Enter")?;
        let found_card_titles = tab.wait_for_elements(
            ".card:not(.filter--hidden) > .card__title-wrapper > .card__title",
        )?;
        assert!(found_card_titles.len() == 2);

        Ok(())
    }

    #[test]
    fn test_get_specific_date() -> Result<(), failure::Error> {
        let browser = Browser::default()?;
        let tab = browser.wait_for_initial_tab()?;
        tab.navigate_to("http://localhost:4000/2021/11/30")?;

        // enter search query
        let date_text = tab
            .wait_for_element("#hero > h2")?
            .call_js_fn("function() { return this.innerText }", false)?
            .value
            .unwrap();
        assert!(date_text == "Tuesday, November 30, 2021");

        Ok(())
    }

    #[test]
    fn test_enter_new_date() -> Result<(), failure::Error> {
        let browser = Browser::default()?;
        let tab = browser.wait_for_initial_tab()?;
        tab.navigate_to("http://localhost:4000")?;

        tab.wait_for_element("#date-text-field-input")?.click()?;
        tab.type_str("11")?.type_str("30")?;
        tab.wait_for_element("#date-submit-button")?.click()?;

        let date_text = tab
            .wait_for_element("#hero > h2")?
            .call_js_fn("function() { return this.innerText }", false)?
            .value
            .unwrap();

        assert!(date_text == "Tuesday, November 30, 2021");

        Ok(())
    }
}

fn main() {}

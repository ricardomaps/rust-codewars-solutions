pub fn page_digits(mut pages: u64) -> u64 {
    let mut res = 0;
    let mut digits_per_page = 1;
    let mut pages_with_digits = 9;

    while pages > 0 {
        let count_of_pages = pages_with_digits.min(pages);
        res += digits_per_page * count_of_pages;
        pages -= count_of_pages;
        pages_with_digits *= 10;
        digits_per_page += 1;
    }
    res
}

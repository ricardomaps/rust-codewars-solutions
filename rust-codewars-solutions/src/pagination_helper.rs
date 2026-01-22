struct PaginationHelper<T> {
    // allows the program to compile, but remove after adding your fields
    collection: Vec<T>,
    items_per_page: usize,
}
// let helper = PaginationHelper::new(vec!['a', 'b', 'c', 'd', 'e', 'f'], 4);
// helper.page_count() // should be 2
// helper.item_count() // should be 6
// helper.page_item_count(0) // should be Some(4)
// helper.page_item_count(1) // should be Some(2) (last page)
// helper.page_item_count(2) // should be None (since the page is invalid)

// // page_index takes an item index and returns the page that it belongs on
// helper.page_index(5) // should be Some(1) (zero based index)
// helper.page_index(2) // should be Some(0)
// helper.page_index(20) // should be None (since the item is invalid)

impl<T> PaginationHelper<T> {
    fn new(collection: Vec<T>, items_per_page: usize) -> Self {
        PaginationHelper {
            collection,
            items_per_page,
        }
    }

    fn item_count(&self) -> usize {
        self.collection.len()
    }

    fn page_count(&self) -> usize {
        (self.item_count() + self.items_per_page - 1) / self.items_per_page
    }

    fn page_item_count(&self, page_index: usize) -> Option<usize> {
        (page_index < self.page_count()).then(|| {
            if page_index == self.page_count() - 1 {
                let rem = self.item_count() % self.items_per_page;
                if rem == 0 { self.items_per_page } else { rem }
            } else {
                self.items_per_page
            }
        })
    }

    fn page_index(&self, item_index: usize) -> Option<usize> {
        (item_index < self.item_count()).then(|| item_index / self.items_per_page)
    }
}

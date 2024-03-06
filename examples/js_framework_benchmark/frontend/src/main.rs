use rand::prelude::*;
use std::sync::{
    atomic::{AtomicUsize, Ordering},
    Arc,
};
use std::iter::repeat_with;
use zoon::{format, *};

type ID = usize;

static ADJECTIVES: &[&'static str] = &[
    "pretty",
    "large",
    "big",
    "small",
    "tall",
    "short",
    "long",
    "handsome",
    "plain",
    "quaint",
    "clean",
    "elegant",
    "easy",
    "angry",
    "crazy",
    "helpful",
    "mushy",
    "odd",
    "unsightly",
    "adorable",
    "important",
    "inexpensive",
    "cheap",
    "expensive",
    "fancy",
];

static COLOURS: &[&'static str] = &[
    "red", "yellow", "blue", "green", "pink", "brown", "purple", "brown", "white", "black",
    "orange",
];

static NOUNS: &[&'static str] = &[
    "table", "chair", "house", "bbq", "desk", "car", "pony", "cookie", "sandwich", "burger",
    "pizza", "mouse", "keyboard",
];

static NEXT_ID: AtomicUsize = AtomicUsize::new(1);
static SELECTED_ROW: Lazy<Mutable<Option<ID>>> = lazy::default();
static ROWS: Lazy<MutableVec<Arc<Row>>> = lazy::default();

struct Row {
    id: ID,
    label: Mutable<String>,
}

fn create_row() -> Arc<Row> {
    let mut generator = SmallRng::from_entropy();
    let label = format!(
        "{} {} {}",
        ADJECTIVES.choose(&mut generator).unwrap_throw(),
        COLOURS.choose(&mut generator).unwrap_throw(),
        NOUNS.choose(&mut generator).unwrap_throw(),
    );
    Arc::new(Row {
        id: NEXT_ID.fetch_add(1, Ordering::SeqCst),
        label: Mutable::new(label),
    })
}

fn create_rows(count: usize) {
    ROWS.lock_mut()
        .replace_cloned(repeat_with(create_row).take(count).collect())
}

fn append_rows(count: usize) {
    ROWS.lock_mut().extend(repeat_with(create_row).take(count));
}

fn update_rows(step: usize) {
    let rows = ROWS.lock_ref();
    for position in (0..rows.len()).step_by(step) {
        rows[position].label.lock_mut().push_str(" !!!");
    }
}

fn clear_rows() {
    ROWS.lock_mut().clear()
}

fn swap_rows() {
    let mut rows = ROWS.lock_mut();
    if rows.len() < 999 {
        return;
    }
    rows.swap(1, 998)
}

fn select_row(id: ID) {
    SELECTED_ROW.set(Some(id))
}

fn remove_row(id: ID) {
    ROWS.lock_mut().retain(|row| row.id != id);
}

fn main() {
    start_app("main", root);
}

fn root() -> impl Element {
    RawHtmlEl::new("div")
        .attr("class", "container")
        .child(jumbotron())
        .child(table())
        .child(
            RawHtmlEl::new("span")
                .attr("class", "preloadicon glyphicon glyphicon-remove")
                .attr("aria-hidden", ""),
        )
}

fn jumbotron() -> impl Element {
    RawHtmlEl::new("div").attr("class", "jumbotron").child(
        RawHtmlEl::new("div").attr("class", "row").children([
            RawHtmlEl::new("div")
                .attr("class", "col-md-6")
                .child(RawHtmlEl::new("h1").child("MoonZoon")),
            RawHtmlEl::new("div")
                .attr("class", "col-md-6")
                .child(action_buttons()),
        ]),
    )
}

fn action_buttons() -> impl Element {
    RawHtmlEl::new("div").attr("class", "row").children([
        action_button("run", "Create 1,000 rows", || create_rows(1_000)),
        action_button("runlots", "Create 10,000 rows", || create_rows(10_000)),
        action_button("add", "Append 1,000 rows", || append_rows(1_000)),
        action_button("update", "Update every 10th row", || update_rows(10)),
        action_button("clear", "Clear", clear_rows),
        action_button("swaprows", "Swap Rows", swap_rows),
    ])
}

fn action_button(id: &'static str, title: &'static str, on_click: fn()) -> impl Element {
    RawHtmlEl::new("div")
        .attr("class", "col-sm-6 smallpad")
        .child(
            RawHtmlEl::new("button")
                .attr("id", id)
                .attr("class", "btn btn-primary btn-block")
                .attr("type", "button")
                .event_handler(move |_: events::Click| on_click())
                .child(title),
        )
}

fn table() -> impl Element {
    RawHtmlEl::new("table")
        .attr("class", "table table-hover table-striped test-data")
        .child_signal(ROWS.signal_vec_cloned().is_empty().map_false(|| {
            RawHtmlEl::new("tbody")
                .attr("id", "tbody")
                .children_signal_vec(ROWS.signal_vec_cloned().map(row))
        }))
}

fn row(row: Arc<Row>) -> impl Element {
    let id = row.id;
    RawHtmlEl::new("tr")
        .attr_signal(
            "class",
            SELECTED_ROW.signal_ref(move |selected_id| ((*selected_id)? == id).then(|| "danger")),
        )
        .child(row_id(id))
        .child(row_label(id, row.label.signal_cloned()))
        .child(row_remove_button(id))
        .child(RawHtmlEl::new("td").attr("class", "col-md-6"))
}

fn row_id(id: ID) -> impl Element {
    RawHtmlEl::new("td").attr("class", "col-md-1").child(id)
}

fn row_label(id: ID, label: impl Signal<Item = String> + Unpin + 'static) -> impl Element {
    RawHtmlEl::new("td").attr("class", "col-md-4").child(
        RawHtmlEl::new("a")
            .event_handler(move |_: events::Click| select_row(id))
            .child(Text::with_signal(label)),
    )
}

fn row_remove_button(id: ID) -> impl Element {
    RawHtmlEl::new("td").attr("class", "col-md-1").child(
        RawHtmlEl::new("a")
            .event_handler(move |_: events::Click| remove_row(id))
            .child(
                RawHtmlEl::new("span")
                    .attr("class", "glyphicon glyphicon-remove remove")
                    .attr("aria-hidden", "true"),
            ),
    )
}

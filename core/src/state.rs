enum Lifecycle {
    ACTIVE,
    OBSOLETE,
}

struct DataState {
    id: u128,
    lifecycle: Lifecycle,

}
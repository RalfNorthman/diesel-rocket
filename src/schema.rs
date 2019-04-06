table! {
    measurements (id) {
        id -> Unsigned<Bigint>,
        temperature -> Double,
        humidity -> Double,
        pressure -> Double,
        comment -> Tinytext,
    }
}

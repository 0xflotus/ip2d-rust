@test "can run our script" {
    ./target/release/ip2d -h
}

@test "convert the right number" {
    OUTPUT=$(./target/release/ip2d 1.1.1.1)
    [ "$OUTPUT" -eq 16843009 ]
}

@test "convert the right hex number" {
    OUTPUT=$(./target/release/ip2d 1.1.1.1 -x)
    [ "$OUTPUT" = "0x01010101" ]
}

@test "convert the right ip" {
    OUTPUT=$(./target/release/ip2d -r 16843009)
    [ "$OUTPUT" = "1.1.1.1" ]
}
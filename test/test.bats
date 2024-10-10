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

@test "convert the right ipv6" {
    OUTPUT=$(./target/release/ip2d -6 -r 16843009)
    [ "$OUTPUT" = "::101:101" ]
}

@test "convert the right number from v6" {
    OUTPUT=$(./target/release/ip2d ::101:101)
    [ "$OUTPUT" = "16843009" ]
}

@test "convert the right hex number from v6 ::1" {
    OUTPUT=$(./target/release/ip2d ::1 -x -6)
    [ "$OUTPUT" = "0x00000000000000000000000000000001" ]
}

@test "convert the right hex number from v6 ffff:34::1" {
    OUTPUT=$(./target/release/ip2d ffff:34::1 -x -6)
    [ "$OUTPUT" = "0xffff0034000000000000000000000001" ]
}

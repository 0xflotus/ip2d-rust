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

@test "convert IPv6 with leading zero blocks" {
    OUTPUT=$(./target/release/ip2d 0000:0000:0000:0000:0000:0000:0000:0001 -6)
    [ "$OUTPUT" = "1" ]
}

@test "convert from IPv6 with trailing zero blocks" {
    OUTPUT=$(./target/release/ip2d 2001:0db8:: -6)
    [ "$OUTPUT" = "42540766411282592856903984951653826560" ]
}

@test "convert IPv6 with mixed zero and non-zero blocks" {
    OUTPUT=$(./target/release/ip2d 2001:0db8:85a3:0000:0000:8a2e:0370:7334 -6)
    [ "$OUTPUT" = "42540766452641154071740215577757643572" ]
}

@test "convert IPv6 with full blocks of ffff" {
    OUTPUT=$(./target/release/ip2d ffff:ffff:ffff:ffff:ffff:ffff:ffff:ffff -6)
    [ "$OUTPUT" = "340282366920938463463374607431768211455" ]
}

@test "convert minimal IPv6 ::" {
    OUTPUT=$(./target/release/ip2d :: -6)
    [ "$OUTPUT" = "0" ]
}

@test "convert IPv6 with a single block ::abcd" {
    OUTPUT=$(./target/release/ip2d ::abcd -6)
    [ "$OUTPUT" = "43981" ]
}

@test "convert IPv6 with embedded IPv4 address to decimal" {
    OUTPUT=$(./target/release/ip2d ::ffff:192.168.1.1 -6)
    [ "$OUTPUT" = "281473913979137" ]
}

@test "convert IPv6 with a single block ::abcd to hex" {
    OUTPUT=$(./target/release/ip2d ::abcd -6 -x)
    [ "$OUTPUT" = "0x0000000000000000000000000000abcd" ]
}

@test "convert large IPv6 address to hex" {
    OUTPUT=$(./target/release/ip2d 2001:0db8:1234:5678:abcd:ef01:2345:6789 -x -6)
    [ "$OUTPUT" = "0x20010db812345678abcdef0123456789" ]
}

@test "convert IPv6 with embedded IPv4 address" {
    OUTPUT=$(./target/release/ip2d ::ffff:192.168.1.1 -x -6)
    [ "$OUTPUT" = "0x00000000000000000000ffffc0a80101" ]
}

@test "convert decimal to address IPv6" {
    OUTPUT=$(./target/release/ip2d -6 -r 281473913979137)
    [ "$OUTPUT" = "::ffff:192.168.1.1" ]
}

@test "convert another decimal to address IPv6" {
    OUTPUT=$(./target/release/ip2d -6 -r 191085225705473)
    [ "$OUTPUT" = "::adca:7f00:1" ]
}
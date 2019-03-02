# Byte Struct

Pack and unpack structure as raw bytes with packed or bit field layout.

## Example
```rust
use byte_struct::*;

bitfields!(
    #[derive(PartialEq, Debug)]
    GIFColorTableInfo: u8 {
        global_color_table_flag: 1,
        color_resolution: 3,
        sort_flag: 1,
        global_color_table_size: 3,
    }
);

#[derive(ByteStructLE, PartialEq, Debug)]
struct GIFLogicalScreenDescriptor {
    width: u16,
    height: u16,
    color_table_info: GIFColorTableInfo,
    background_color_index: u8,
    pixel_aspect_ratio: u8,
}

fn example() {
    assert_eq!(GIFLogicalScreenDescriptor::BYTE_LEN, 7);
    let raw_descriptor = [0x03, 0x00, 0x05, 0x00, 0xF7, 0x00, 0x00];
    let descriptor = GIFLogicalScreenDescriptor::read_bytes(&raw_descriptor[..]);
    assert_eq!(descriptor, GIFLogicalScreenDescriptor{
        width: 3,
        height: 5,
        color_table_info: GIFColorTableInfo {
            global_color_table_flag: 1,
            color_resolution: 3,
            sort_flag: 1,
            global_color_table_size: 7,
        },
        background_color_index: 0,
        pixel_aspect_ratio: 0,
    });
    let mut raw_another = [0; GIFLogicalScreenDescriptor::BYTE_LEN];
    descriptor.write_bytes(&mut raw_another[..]);
    assert_eq!(raw_descriptor, raw_another);
}
```

## License

Licensed under either of

 * Apache License, Version 2.0, ([LICENSE-APACHE](LICENSE-APACHE) or http://www.apache.org/licenses/LICENSE-2.0)
 * MIT license ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT)

at your option.

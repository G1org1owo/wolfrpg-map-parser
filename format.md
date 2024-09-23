## Map format

| Length                 | Content        | Value                                                                        |
|------------------------|----------------|------------------------------------------------------------------------------|
| 18 bytes               | WOLF signature | `00 00 00 00 00 00 00 00 00 00 57 4F 4C 46 4D 00 00 00 00 00 64 00 00 00 65` |
| 4 bytes                | offset         | little-endian uint32                                                         |
| `offset` bytes         | ???            | ???                                                                          |
| 4 bytes                | tileset        | little-endian uint32                                                         |
| 4 bytes                | width          | little-endian uint32                                                         |
| 4 bytes                | height         | little-endian uint32                                                         |
| 4 bytes                | event_count    | little-endian uint32                                                         |
| `width*height*4` bytes | layer1         | [little-endian uint32; `width*height`]                                       |
| `width*height*4` bytes | layer2         | [little-endian uint32; `width*height`]                                       |
| `width*height*4` bytes | layer3         | [little-endian uint32; `width*height`]                                       |
| \<rest of file>        | events         | [Event; `event_count`]                                                       |
| 1 byte                 | Map end        | `66`                                                                         |


## Event format

| Length               | Content             | Value                        |
|----------------------|---------------------|------------------------------|
| 9 bytes              | Event signature     | `6F 39 30 00 00 00 00 00 00` |
| 4 bytes              | title_length        | little-endian uint32         |
| `title_length` bytes | title               | NUL-terminated string        |
| 4 bytes              | ???                 | little-endian uint32         |
| 4 bytes              | ???                 | little-endian uint32         |
| 4 bytes              | page_count          | little-endian uint32         |
| 4 bytes              | ???                 | little-endian uint32         |
| 5 bytes              | Icon signature      | `79 FF FF FF FF`             |
| 4 bytes              | icon_length         | little-endian uint32         |
| `icon_length` bytes  | icon_name           | NUL-terminated string        |
| 1 byte               | icon_row            | uint8                        |
| 1 byte               | icon_column         | uint8                        |
| 1 byte               | icon_opacity        | uint8                        |
| 1 byte               | icon_blend          | uint8                        |
| 1 byte               | event_trigger       | uint8                        |
| 1 byte               | condition1_operator | uint8 bitmask                |
| 1 byte               | condition2_operator | uint8 bitmask                |
| 1 byte               | condition3_operator | uint8 bitmask                |
| 1 byte               | condition4_operator | uint8 bitmask                |
| 4 byte               | condition1_variable | little-endian uint32         |
| 4 byte               | condition2_variable | little-endian uint32         |
| 4 byte               | condition3_variable | little-endian uint32         |
| 4 byte               | condition4_variable | little-endian uint32         |
| 4 byte               | condition1_value    | little-endian uint32         |
| 4 byte               | condition2_value    | little-endian uint32         |
| 4 byte               | condition3_value    | little-endian uint32         |
| 4 byte               | condition4_value    | little-endian uint32         |
| 1 byte               | animation_speed     | uint8                        |
| 1 byte               | move_speed          | uint8                        |
| 1 byte               | move_frequency      | uint8                        |
| 1 byte               | move_route          | uint8                        |
| 1 byte               | options             | uint8 bitmask                |
| 1 byte               | ???                 |                              |
| 4 byte               | ???                 |                              |
| 4 byte               | command_count       | little-endian uint32         |
| \<variable>          | commands            | [Command; command_count]     |
| ...                  | ...                 | ...                          |
| 1 byte               | shadow_graphic      | uint8                        |
| 1 byte               | range_extension_x   | uint8                        |
| 1 byte               | range_extension_y   | uint8                        |
| 1 byte               | Page end            | `7A`                         |
| 1 byte               | Event end           | `70`                         |

## Command format
| Length      | Content      | Value                |
|-------------|--------------|----------------------|
| 4 byte      | command_code | little-endian uint32 |
| \<variable> | command_data | \<variable>          |

### Show message Command format

| Length                 | Content        | Value                 |
|------------------------|----------------|-----------------------|
| 4 byte                 | command_code   | `01 65 00 00`         |
| 1 byte                 | ???            | uint8                 |
| 1 byte                 | ???            | uint8                 |
| 1 byte                 | ???            | uint8                 |
| 4 bytes                | message_length | little-endian uint32  |
| `message_length` bytes | message_length | NUL-terminated string |
| 1 byte                 | Command end    | `00`                  |

### Exit Command format
| Length                 | Content      | Value                 |
|------------------------|--------------|-----------------------|
| 4 byte                 | command_code | `01 65 00 00`         |
| 4 bytes                | ???          | little-endian uint32  |
| 4 bytes                | ???          | little-endian uint32  |

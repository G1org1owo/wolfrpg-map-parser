# Map format

| Length                 | Content        | Value                                                                        |
|------------------------|----------------|------------------------------------------------------------------------------|
| 25 bytes               | WOLF signature | `00 00 00 00 00 00 00 00 00 00 57 4F 4C 46 4D 00 00 00 00 00 64 00 00 00 65` |
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


# Event format

| Length               | Content             | Value                    |
|----------------------|---------------------|--------------------------|
| 5 bytes              | Event signature     | `6F 39 30 00`            |
| 4 bytes              | event_id            |                          |
| 4 bytes              | title_length        | little-endian uint32     |
| `title_length` bytes | title               | NUL-terminated string    |
| 4 bytes              | position_x          | little-endian uint32     |
| 4 bytes              | position_y          | little-endian uint32     |
| 4 bytes              | page_count          | little-endian uint32     |
| 4 bytes              | ???                 | little-endian uint32     |
| 5 bytes              | Icon signature      | `79 FF FF FF FF`         |
| 4 bytes              | icon_length         | little-endian uint32     |
| `icon_length` bytes  | icon_name           | NUL-terminated string    |
| 1 byte               | icon_row            | uint8                    |
| 1 byte               | icon_column         | uint8                    |
| 1 byte               | icon_opacity        | uint8                    |
| 1 byte               | icon_blend          | uint8                    |
| 1 byte               | event_trigger       | uint8                    |
| 1 byte               | condition1_operator | uint8 bitmask            |
| 1 byte               | condition2_operator | uint8 bitmask            |
| 1 byte               | condition3_operator | uint8 bitmask            |
| 1 byte               | condition4_operator | uint8 bitmask            |
| 4 bytes              | condition1_variable | little-endian uint32     |
| 4 bytes              | condition2_variable | little-endian uint32     |
| 4 bytes              | condition3_variable | little-endian uint32     |
| 4 bytes              | condition4_variable | little-endian uint32     |
| 4 bytes              | condition1_value    | little-endian uint32     |
| 4 bytes              | condition2_value    | little-endian uint32     |
| 4 bytes              | condition3_value    | little-endian uint32     |
| 4 bytes              | condition4_value    | little-endian uint32     |
| 1 byte               | animation_speed     | uint8                    |
| 1 byte               | move_speed          | uint8                    |
| 1 byte               | move_frequency      | uint8                    |
| 1 byte               | move_route          | uint8                    |
| 1 byte               | options             | uint8 bitmask            |
| 1 byte               | ???                 |                          |
| 4 bytes              | ???                 |                          |
| 4 bytes              | command_count       | little-endian uint32     |
| \<variable>          | commands            | [Command; command_count] |
| 4 bytes              | ???                 |                          |
| 1 byte               | shadow_graphic      | uint8                    |
| 1 byte               | range_extension_x   | uint8                    |
| 1 byte               | range_extension_y   | uint8                    |
| 1 byte               | Page end            | `7A`                     |
| 1 byte               | Event end           | `70`                     |

# Command format
| Length      | Content      | Value                |
|-------------|--------------|----------------------|
| 4 bytes     | command_code | little-endian uint32 |
| \<variable> | command_data | \<variable>          |

## Show message Command format

| Length                 | Content        | Value                 |
|------------------------|----------------|-----------------------|
| 4 bytes                | command_code   | `01 65 00 00`         |
| 1 byte                 | ???            | uint8                 |
| 1 byte                 | ???            | uint8                 |
| 1 byte                 | ???            | uint8                 |
| 4 bytes                | message_length | little-endian uint32  |
| `message_length` bytes | message_length | NUL-terminated string |
| 1 byte                 | Command end    | `00`                  |

## Show choice Command format
| Length      | Content          | Value                                                                        |
|-------------|------------------|------------------------------------------------------------------------------|
| 4 bytes     | command_code     | `02 66 00 00`                                                                |
| 1 byte      | ???              |                                                                              |
| 1 byte      | selected_choices | high 4 bits for default choice, low 4 bits for choice count                  |
| 1 byte      | extra_cases      | uint8 bitmap                                                                 |
| 2 bytes     | ???              |                                                                              |
| 1 byte      | ???              |                                                                              |
| 1 byte      | choice_count     | uint8                                                                        |
| \<variable> | choices          | [Choice; choice_count]                                                       |
| 1 byte      | Choices end      | `00`                                                                         |
| \<variable> | Cases            | [Case; choice_count + extra_cases (+ 1 if selected_choices high bits are 0)] |
| 4 bytes     | ???              | `01 F3 00 00`                                                                |
| 4 bytes     | ???              |                                                                              |

### Choice format
| Length              | Content       | Value                 |
|---------------------|---------------|-----------------------|
| 4 bytes             | choice_length | little-endian uint32  |
| `case_length` bytes | choice        | NUL-terminated string |

### Case format
| Length      | Content      | Value                                         |
|-------------|--------------|-----------------------------------------------|
| 4 bytes     | command_code | `02 91 01 00`, `02 92 01 00` or `02 A5 01 00` |
| 1 byte      | ???          |                                               |
| 1 byte      | case_id      | uint8                                         |
| 2 bytes     | ???          |                                               |
| 4 bytes     | ???          |                                               |
| \<variable> | commands     | [Command]                                     |
| 8 bytes     | Exit         |                                               |

## Set variable Command
### Base variant
| Length  | Content      | Value                |
|---------|--------------|----------------------|
| 4 bytes | command_code | `05 79 00 00`        |
| 1 byte  | ???          |                      |
| 4 bytes | variable     | little-endian uint32 |
| 4 bytes | left_side    | little-endian uint32 |
| 1 byte  | options      | uint8 bitmap         |
| 1 byte  | operators    | uint8 bitmap         |
| 4 bytes | ???          |                      |
| 1 byte  | Command end  | `00`                 |

### Range variant
| Length  | Content      | Value                |
|---------|--------------|----------------------|
| 4 bytes | command_code | `05 79 00 00`        |
| 1 byte  | ???          |                      |
| 4 bytes | variable     | little-endian uint32 |
| 4 bytes | left_side    | little-endian uint32 |
| 1 byte  | options      | uint8 bitmap         |
| 1 byte  | operators    | uint8 bitmap         |
| 2 bytes | range_start  | little-endian uint16 |
| 2 bytes | range_end    | little-endian uint16 |
| 4 bytes | ???          |                      |
| 1 byte  | Command end  | `00`                 |

## Exit Command format
| Length  | Content      | Value         |
|---------|--------------|---------------|
| 4 bytes | command_code | `01 65 00 00` |
| 2 bytes | ???          | uint16        |
| 2 bytes | ???          | uint16        |

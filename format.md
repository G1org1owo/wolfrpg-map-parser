This is a format specification I have put together while decoding the .mps map files, as such it is not complete nor
very in-depth. I advise the ImHex pattern files located in `extra/wolf` be used as reference for a more accurate analysis. 

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

## Set variable Command format
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

## DB management Command format
### Base Variant
| Length                 | Content        | Value                 |
|------------------------|----------------|-----------------------|
| 4 bytes                | command_code   | `06 FA 00 00`         |
| 1 byte                 | ???            |                       |
| 4 bytes                | db_type        | little-endian uint32  |
| 4 bytes                | data           | little-endian uint32  |
| 4 bytes                | field          | little-endian uint32  |
| 1 byte                 | assignment     | uint8 bitmap          |
| 1 byte                 | options        | uint8 bitmap          |
| 2 bytes                | ???            |                       |
| 4 bytes                | value          | little-endian uint32  |
| 1 byte                 | ???            |                       |
| 1 byte                 | string_count   | uint8                 |
| 4 bytes                | value_length   | little-endian uint32  |
| `value_length` bytes   | value_string   | NUL-terminated string |
| 4 bytes                | db_type_length | little-endian uint32  |
| `db_type_length` bytes | db_type_name   | NUL-terminated string |
| 4 bytes                | data_length    | little-endian uint32  |
| `data_length` bytes    | data_name      | NUL-terminated string |
| 4 bytes                | field_length   | little-endian uint32  |
| `field_length` bytes   | field_name     | NUL-terminated string |
| 1 byte                 | Command end    | `00`                  |

### String Variant
| Length                 | Content        | Value                 |
|------------------------|----------------|-----------------------|
| 4 bytes                | command_code   | `05 FA 00 00`         |
| 1 byte                 | ???            |                       |
| 4 bytes                | db_type        | little-endian uint32  |
| 4 bytes                | data           | little-endian uint32  |
| 4 bytes                | field          | little-endian uint32  |
| 1 byte                 | assignment     | uint8 bitmap          |
| 1 byte                 | options        | uint8 bitmap          |
| 2 bytes                | ???            |                       |
| 1 byte                 | ???            |                       |
| 1 byte                 | string_count   | uint8                 |
| 4 bytes                | value_length   | little-endian uint32  |
| `value_length` bytes   | value          | NUL-terminated string |
| 4 bytes                | db_type_length | little-endian uint32  |
| `db_type_length` bytes | db_type_name   | NUL-terminated string |
| 4 bytes                | data_length    | little-endian uint32  |
| `data_length` bytes    | data_name      | NUL-terminated string |
| 4 bytes                | field_length   | little-endian uint32  |
| `field_length` bytes   | field_name     | NUL-terminated string |
| 1 byte                 | Command end    | `00`                  |

### CSV Variant
| Length                  | Content         | Value                 |
|-------------------------|-----------------|-----------------------|
| 4 bytes                 | command_code    | `06 FB 00 00`         |
| 1 byte                  | ???             |                       |
| 4 bytes                 | db_type         | little-endian uint32  |
| 4 bytes                 | data            | little-endian uint32  |
| 4 bytes                 | field           | little-endian uint32  |
| 1 byte                  | assignment      | uint8 bitmap          |
| 1 byte                  | options         | uint8 bitmap          |
| 2 bytes                 | ???             |                       |
| 4 bytes                 | entry_count     | little-endian uint32  |
| 1 byte                  | ???             |                       |
| 1 byte                  | string_count    | uint8                 |
| 4 bytes                 | filename_length | little-endian uint32  |
| `filename_length` bytes | filename        | NUL-terminated string |
| 4 bytes                 | db_type_length  | little-endian uint32  |
| `db_type_length` bytes  | db_type_name    | NUL-terminated string |
| 4 bytes                 | data_length     | little-endian uint32  |
| `data_length` bytes     | data_name       | NUL-terminated string |
| 4 bytes                 | field_length    | little-endian uint32  |
| `field_length` bytes    | field_name      | NUL-terminated string |
| 1 byte                  | Command end     | `00`                  |

## Set string Command
### Base Variant
| Length  | Content        | Value                |
|---------|----------------|----------------------|
| 4 bytes | command_code   | `03 7A 00 00`        |
| 1 byte  | ???            |                      |
| 4 bytes | variable       | little-endian uint32 |
| 1 byte  | options        | uint8 bitmap         |
| 1 byte  | operation      | uint8 bitmap         |
| 2 bytes | ???            |                      |
| 1 byte  | ???            |                      |
| 1 byte  | string_count   | uint8                |
| 4 bytes | string_length  | little-endian uint32 |
| 1 byte  | replace_count  | uint8                |
| 4 bytes | replace_length | little-endian uint32 |
| 1 byte  | Command end    | `00`                 |

### Dynamic Variant
| Length  | Content             | Value                |
|---------|---------------------|----------------------|
| 4 bytes | command_code        | `04 7A 00 00`        |
| 1 byte  | ???                 |                      |
| 4 bytes | variable            | little-endian uint32 |
| 1 byte  | options             | uint8 bitmap         |
| 1 byte  | operation           | uint8 bitmap         |
| 2 bytes | ???                 |                      |
| 4 bytes | source/input_length | little-endian uint32 |
| 2 bytes | ???                 |                      |
| 1 byte  | Command end         | `00`                 |

## Set variable+ Command format
### Character Variant
| Length  | Content      | Value                |
|---------|--------------|----------------------|
| 4 bytes | command_code | `05 7C 00 00`        |
| 1 byte  | ???          |                      |
| 4 bytes | variable     | little-endian uint32 |
| 1 byte  | options      | uint8 bitmap         |
| 1 byte  | assignment   | uint8 bitmap         |
| 2 bytes | ???          |                      |
| 4 bytes | character    | little-endian uint32 |
| 4 bytes | field        | little-endian uint32 |
| 2 bytes | ???          |                      |
| 1 byte  | Command end  | `00`                 |

### Position Variant
| Length  | Content      | Value                |
|---------|--------------|----------------------|
| 4 bytes | command_code | `05 7C 00 00`        |
| 1 byte  | ???          |                      |
| 4 bytes | variable     | little-endian uint32 |
| 1 byte  | options      | uint8 bitmap         |
| 1 byte  | assignment   | uint8 bitmap         |
| 1 byte  | target       | uint8                |
| 1 byte  | ???          |                      |
| 4 bytes | position_x   | little-endian uint32 |
| 4 bytes | position_y   | little-endian uint32 |
| 2 bytes | ???          |                      |
| 1 byte  | Command end  | `00`                 |

### Picture number Variant
| Length  | Content        | Value                |
|---------|----------------|----------------------|
| 4 bytes | command_code   | `05 7C 00 00`        |
| 1 byte  | ???            |                      |
| 4 bytes | variable       | little-endian uint32 |
| 1 byte  | options        | uint8 bitmap         |
| 1 byte  | assignment     | uint8 bitmap         |
| 2 bytes | ???            |                      |
| 4 bytes | picture_number | little-endian uint32 |
| 4 bytes | field          | little-endian uint32 |
| 2 bytes | ???            |                      |
| 1 byte  | Command end    | `00`                 |

### Other variant
| Length  | Content      | Value                |
|---------|--------------|----------------------|
| 4 bytes | command_code | `04 7C 00 00`        |
| 1 byte  | ???          |                      |
| 4 bytes | variable     | little-endian uint32 |
| 1 byte  | options      | uint8 bitmap         |
| 1 byte  | assignment   | uint8 bitmap         |
| 2 bytes | ???          |                      |
| 4 bytes | target       | little-endian uint32 |
| 2 bytes | ???          |                      |
| 1 byte  | Command end  | `00`                 |

## Condition number Command format
| Length                  | Content      | Value                                         |
|-------------------------|--------------|-----------------------------------------------|
| 4 bytes                 | command_code | `05 6F 00 00`, `08 6F 00 00` or `0B 6F 00 00` |
| 1 byte                  | ???          |                                               |
| 1 byte                  | case_count   | uint8 bitmap                                  |
| 1 byte                  | padding      | `00`                                          |
| 12 bytes * `case_count` | conditions   | [Condition; case_count]                       |
| 4 bytes                 | ???          |                                               |
| 1 byte                  | ???          |                                               |
| \<variable>             | cases        | [Case; case_count]                            |
| \<variable>             | else_case    | Case? (only if flag is set in case_count)     |
| 4 bytes                 | ???          | `01 F3 00 00`                                 |
| 4 bytes                 | ???          |                                               |

### Condition format
| Length  | Content  | Value                |
|---------|----------|----------------------|
| 2 bytes | ???      |                      |
| 4 bytes | variable | little-endian uint32 |
| 4 bytes | value    | little-endian uint32 |
| 1 byte  | operator | uint8 bitmap         |
| 1 byte  | ???      |                      |

## Condition string Command format
| Length                                            | Content         | Value                                                                                                                    |
|---------------------------------------------------|-----------------|--------------------------------------------------------------------------------------------------------------------------|
| 4 bytes                                           | command_code    | `03 70 00 00`, `04 70 00 00`, `05 70 00 00`, `06 70 00 00`, `07 70 00 00`, `08 70 00 00`. `09 70 00 00` or `0A 70 00 00` |
| 1 byte                                            | ???             |                                                                                                                          |
| 1 byte                                            | case_count      | uint8 bitmap                                                                                                             |
| 3 byte                                            | padding         | `00 00 00`                                                                                                               |
| 5 bytes * `case_count`                            | variables       | [ConditionStringVariable; case_count]                                                                                    |
| 4 bytes * `(command_code >> 24) - 2 - case_count` | values          | [u32; `(command_code >> 24) - 2 - case_count`]                                                                           |
| 1 byte                                            | ???             |                                                                                                                          |
| 1 byte                                            | condition_count | uint8                                                                                                                    |
| \<variable>                                       | conditions      | [ConditionStringCondition; condition_count]                                                                              |
| \<variable>                                       | cases           | [Case; case_count]                                                                                                       |
| \<variable>                                       | else_case       | Case? (only if flag is set in case_count)                                                                                |
| 4 bytes                                           | ???             | `01 F3 00 00`                                                                                                            |
| 4 bytes                                           | ???             |                                                                                                                          |

### Condition string Variable format
| Length  | Content  | Value                |
|---------|----------|----------------------|
| 4 bytes | variable | little-endian uint32 |
| 1 byte  | operator | uint8 bitmap         |

### Condition string Condition format
| Length         | Content | Value                |
|----------------|---------|----------------------|
| 4 bytes        | length  | little-endian uint32 |
| `length` bytes | string  | [u8; length]         |

## Input key Command format
### Base variant
| Length      | Content      | Value                                                      |
|-------------|--------------|------------------------------------------------------------|
| 4 bytes     | command_code | `03 7B 00 00` or `04 7B 00 00`                             |
| 1 byte      | ???          |                                                            |
| 4 bytes     | variable     | little-endian uint32                                       |
| 1 byte      | options      | uint8 bitmap                                               |
| 1 byte      | input_type   | uint8                                                      |
| 2 bytes     | ???          |                                                            |
| \<variable> | key_code     | little-endian uint32, only if `input_type` is `01` or `02` |
| 2 bytes     | ???          |                                                            |
| 1 byte      | Command end  | `00`                                                       |

### Automatic input Variant
| Length   | Content      | Value                                                                                    |
|----------|--------------|------------------------------------------------------------------------------------------|
| 4 bytes  | command_code | `02 7D 00 00`, `03 7D 00 00` or `04 7D 00 00`                                            |
| 1 byte   | ???          |                                                                                          |
| 1 byte   | options      | uint8 bitmap                                                                             |
| 2 bytes  | ???          |                                                                                          |
| 1 byte   | input_type   | uint8                                                                                    |
| 4 bytes? | key_code     | little-endian uint32, only if `input_type` is `10`                                       |
| 4 bytes? | position_x   | little-endian uint32, only if `input_type` is `20` and `options` matches mask `00001000` |
| 4 bytes? | position_y   | little-endian uint32, only if `input_type` is `20` and `options` matches mask `00001000` |
| 4 bytes? | wheel_delta  | little-endian uint32, only if `input_type` is `20` and `options` matches mask `00010000` |
| 4 bytes? | ???          | only if `input_type` is `20` and `options` matches mask `00010000`                       |
| 3 bytes  | Command end  | `00 00 00`                                                                               |

### Input toggle Variant
| Length      | Content        | Value                                                                   |
|-------------|----------------|-------------------------------------------------------------------------|
| 4 bytes     | command_code   | `02 7E 00 00`, `03 7E 00 00`                                            |
| 1 byte      | ???            |                                                                         |
| 1 byte      | inputs         | uint8 bitmap                                                            |
| 1 byte      | enabled_inputs | uint8 bitmap                                                            |
| 1 byte      | ???            |                                                                         |
| 1 byte      | input_type     | uint8 bitmap                                                            |
| \<variable> | key_code       | little-endian uint32, only if `input_type` is `00` and `inputs` is `00` |
| 3 bytes     | Command end    | `00 00 00`                                                              |

## Picture Command format
Quite complex, check `extra/wolf/picture.hexpat`

## Exit Command format
| Length  | Content      | Value         |
|---------|--------------|---------------|
| 4 bytes | command_code | `01 65 00 00` |
| 2 bytes | ???          | uint16        |
| 2 bytes | ???          | uint16        |

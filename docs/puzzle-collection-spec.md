You can create your own puzzle collection by creating a JSON file with the following structure.
Note that any behavior not explicitly described here is subject to change without prior notice or increased version
number.
If you want to add new features or define behavior not described here, please open an issue.
Usually, when loading a malformed file, the application will show an error message and refuse to load the file.
If it does not give an error message, when this specification suggests it should, please open an issue as well.

<!-- @formatter:off -->
```json
{
    "spec": 1,
    "name": "Example Puzzle",
    "author": "Puzzle More Days",
    "description": "Fill the board with trominoes.",
    "custom_tiles": {
        "looong": [
            [1, 1, 1]
        ]
    },
    "custom_boards": {
        "3x3": {
            "layout": [
                [0, 0, 0],
                [0, 0, 0],
                [0, 0, 0]
            ]
        }
    },
    "puzzles": [
        {
            "name": "Simple",
            "tiles": [
                "looong",
                "L3",
                [
                    [1, 1],
                    [0, 1]
                ]
            ],
            "board": "3x3"
        }
    ]
}
```
<!-- @formatter:on -->

This file can be loaded by Puzzle More Days to add a new puzzle collection for the user to solve.
The fields have the following meaning:

| Field         | Type                 | Required | Description                                                                                                                                                                 | Default   |
|---------------|----------------------|----------|-----------------------------------------------------------------------------------------------------------------------------------------------------------------------------|-----------|
| puzzled       | `String`             | true     | The version of Puzzled this collection is made for. If you try to load a collection into Puzzled which is made for a newer versin of Puzzled, the load operation will fail. | -         |
| name          | `String`             | true     | The name of the puzzle collection. MUST not be blank. The name SHOULD be short (5 to 20 characters) and not contain the word `Collection`.                                  | -         |
| author        | `String`             | true     | The author of the puzzle collection. MUST not be blank.                                                                                                                     | -         |
| version       | `String`             | false    | The version of the collection set by the author. This may be any string. This is not checked, it is just displayed to the user.                                             | -         |
| description   | `String`             | false    | A short description of the puzzle collection. MUST not be blank if specified.                                                                                               | None      |
| custom_tiles  | `Map<String, Tile>`  | false    | A map of custom tile definitions to reuse in this file. See [Custom Tiles](#custom-tiles) for details.                                                                      | Empty Map |
| custom_boards | `Map<String, Board>` | false    | A map of custom board definitions to reuse in this file. See [Custom Boards](#custom-boards) for details.                                                                   | Empty Map |
| puzzles       | `List<Puzzle>`       | true     | The list of puzzles in this collection. See [Puzzles](#puzzles) for details.                                                                                                | -         |

## Custom Tiles

Custom tiles can be used to define tiles that are not part of the standard tile set.
They can then be referenced in the `tiles` field of a puzzle by their name.
The value of each entry in the `custom_tiles` map is a tile definition, which is explained in the [Tiles](#tiles)
section below.
You can also override standard tiles by defining a custom tile with the same name.
Standard tiles are shown below.

## Custom Boards

Custom boards can be used to define boards that you want to reuse in multiple puzzles.
They can then be referenced in the `board` field of a puzzle by their name.
The value of each entry in the `custom_boards` map is a board definition, which is explained in the [Board](#board)
section below.

## Puzzles

A puzzle describes a single challenge for the user to solve.
It has the following fields:

| Field           | Type                  | Required | Description                                                                                                                                                                                                 | Default   |
|-----------------|-----------------------|----------|-------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------|-----------|
| name            | `String`              | true     | The name of the puzzle. MUST not be blank.                                                                                                                                                                  | -         |
| description     | `String`              | false    | A short description of the puzzle. MUST not be blank if specified.                                                                                                                                          | None      |
| difficulty      | `Difficulty`          | false    | The difficulty of the puzzle. If provided, it MUST be one of: `Easy`, `Medium`, `Hard` or `Expert`.                                                                                                         | None      |
| tiles           | `List<Tile>`          | true     | The list of tiles available to solve the puzzle. This list MUST not be empty.                                                                                                                               | -         |
| board           | `Board`               | true     | The board to solve the puzzle on. See below for details.                                                                                                                                                    | -         |   
| additional_info | `Map<String, String>` | false    | Additional information about the puzzle. This may contain statistics about how many solutions there are, or anything else, which is in a key-value format and does not feel right to be in the description. | Empty Map |   

## Tiles

A tile defines a shape that can be placed on the board.
Tiles can be defined in two ways: by name or with an array.

### Name

A tile can be referenced by its name as a `String`.
Those tiles are either standard tiles (see below) or custom tiles defined in the `custom_tiles` map.

Example:

```json
"L3"
```

The following standard tiles are available:

| Name            | Shape                                                               |
|-----------------|---------------------------------------------------------------------|
| **Trominoes**   |                                                                     |
| `I3`            | ![I3 Tile](https://til7701.de/assets/images/puzzled/tiles/3/I3.svg) |
| `L3`            | ![L3 Tile](https://til7701.de/assets/images/puzzled/tiles/3/L3.svg) |
| **Tetrominoes** |                                                                     |
| `I4`            | ![I4 Tile](https://til7701.de/assets/images/puzzled/tiles/4/I4.svg) |
| `J4`            | ![L4 Tile](https://til7701.de/assets/images/puzzled/tiles/4/J4.svg) |
| `L4`            | ![L4 Tile](https://til7701.de/assets/images/puzzled/tiles/4/L4.svg) |
| `O4`            | ![O4 Tile](https://til7701.de/assets/images/puzzled/tiles/4/O4.svg) |
| `S4`            | ![S4 Tile](https://til7701.de/assets/images/puzzled/tiles/4/S4.svg) |
| `T4`            | ![T4 Tile](https://til7701.de/assets/images/puzzled/tiles/4/T4.svg) |
| `Z4`            | ![Z4 Tile](https://til7701.de/assets/images/puzzled/tiles/4/Z4.svg) |
| **Pentominoes** |                                                                     |
| `F5`            | ![F5 Tile](https://til7701.de/assets/images/puzzled/tiles/5/F5.svg) |
| `I5`            | ![I5 Tile](https://til7701.de/assets/images/puzzled/tiles/5/I5.svg) |
| `L5`            | ![L5 Tile](https://til7701.de/assets/images/puzzled/tiles/5/L5.svg) |
| `N5`            | ![N5 Tile](https://til7701.de/assets/images/puzzled/tiles/5/N5.svg) |
| `P5`            | ![P5 Tile](https://til7701.de/assets/images/puzzled/tiles/5/P5.svg) |
| `T5`            | ![T5 Tile](https://til7701.de/assets/images/puzzled/tiles/5/T5.svg) |
| `U5`            | ![U5 Tile](https://til7701.de/assets/images/puzzled/tiles/5/U5.svg) |
| `V5`            | ![V5 Tile](https://til7701.de/assets/images/puzzled/tiles/5/V5.svg) |
| `W5`            | ![W5 Tile](https://til7701.de/assets/images/puzzled/tiles/5/W5.svg) |
| `X5`            | ![X5 Tile](https://til7701.de/assets/images/puzzled/tiles/5/X5.svg) |
| `Y5`            | ![Y5 Tile](https://til7701.de/assets/images/puzzled/tiles/5/Y5.svg) |
| `Z5`            | ![I3 Tile](https://til7701.de/assets/images/puzzled/tiles/5/Z5.svg) |

### Array

A tile can also be defined by a 2D array of integers.
The array represents the shape of the tile, where `1` indicates a filled cell and `0` indicates an empty cell.

For example, the following array defines a Z5 tile:

<!-- @formatter:off -->
```json
[
    [1, 1, 0],
    [0, 1, 0],
    [0, 1, 1]
]
```
<!-- @formatter:on -->

Which, as a reminder, looks like this:

<img alt="Z5 Tile" src="https://til7701.de/assets/images/puzzled/tiles/5/Z5.svg" height="100"/>

## Board

A board defines the layout on which the puzzle is to be solved.
There are two variants of boards, which can be defined in three ways, so pay attention.
Boards can be referenced by name, defined as a [Simple Board](#simple-board) or as an [Area Board](#area-board).

### Name

Similar to tiles, a board can be referenced by its name as a `String`.
Standard boards or custom boards defined in the `custom_boards` map can be referenced this way.

Example:

```json
"my_custom_board"
```

The following format of standard boards are available:

You can define a simple rectangular board of any size using the following naming scheme:
`<width>x<height>`, where `<width>` and `<height>` are positive integers.
The following board definitions will result in the same board:

```json
"4x3"
```

<!-- @formatter:off -->
```json
{
    "layout": [
        [0, 0, 0, 0],
        [0, 0, 0, 0],
        [0, 0, 0, 0]
    ]
}
```
<!-- @formatter:on -->

### Simple Board

A simple board is defined by a 2D array of integers, where `0` indicates an empty cell where a tile can be placed and
`1` indicates a blocked cell where no tile can be placed.

| Field  | Type         | Required | Description                                                                              | Default   |
|--------|--------------|----------|------------------------------------------------------------------------------------------|-----------|
| layout | `Array2<u8>` | true     | The layout of the board where `0` indicates empty cells and `1` indicates blocked cells. | -         |

Example:

<!-- @formatter:off -->
```json
{
    "layout": [
        [0, 0, 0],
        [0, 1, 0],
        [0, 0, 0]
    ]
}
```
<!-- @formatter:on -->

Here, tiles can be placed in all cells except the one in the center.

### Area Board

An area board is a board, where one cell has to be left empty in each area when solving the puzzle.
This is the board type used in the original Puzzle A Day.

| Field           | Type             | Required | Description                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                             | Default |
|-----------------|------------------|----------|-----------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------|---------|
| area_layout     | `Array2<i32>`    | true     | The layout of the areas on the board. `-1` indicates, that no tile can be placed in this cell. The rest is the index of the area the cell belongs to. The index starts at `0` and goes up to the length of the areas list minus one.                                                                                                                                                                                                                                                                                                    | -       |
| values          | `Array2<String>` | true     | The values to show in each cell. Entries MUST not be empty, when the area_layout is not `-1` in the corresponding cell.                                                                                                                                                                                                                                                                                                                                                                                                                 | -       |
| value_order     | `Array2<i32>`    | true     | The order of values. This order is used to sort the options when selecting a target. `-1` marks cells that no tile can be placed in. The rest of the values MUST be any non-negative number and MAY not start at zero and MAY not be consecutive. The natural order of the number is the only thing that matters. If the same number appears more than once in an area, the behavior is undefined.                                                                                                                                      | -       |
| areas           | `List<Area>`     | true     | The list of [Areas](#area) in this board. The index of the area in this board is the same as the index for the area in the area_layout array.                                                                                                                                                                                                                                                                                                                                                                                           | -       |
| target_template | `String`         | true     | Template to format a target selected by the user. This is shown to inform the user about the currently selected target. The formatters defined in the area are used to format the value from the board values array. In the template, `{<area_index>}` is replaced with the formatted value. So `{0} and {1}` is a template for a baord with two areas. The resulting string may look like this: `first and second`, where `first` is the formatted value from the first area and `second` is the formatted vlaue from the second area. | -       |

Example:

<!-- @formatter:off -->
```json
{
    "area_layout": [
        [0, 0, 0, -1],
        [-1, 1, 1, 1]
    ],
    "values": [
        ["A", "B", "C", ""],
        ["", "D", "E", "F"]
    ],
    "value_order": [
        [0, 1, 2, -1],
        [-1, 0, 1, 2]
    ],
    "areas": [...],
    "target_template": "{0} and {1}"
}
```
<!-- @formatter:on -->

#### Area

The area gives additional information about an area on an area board.

| Field           | Type             | Required | Description                                                                                                                                                                                                                                                           | Default   |
|-----------------|------------------|----------|-----------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------|-----------|
| name            | `String`         | true     | The name of the area to show in the target selection dialog.                                                                                                                                                                                                          | -         |
| formatter       | `AreaFormatter`  | true     | The [formatter](#area-formatter) that defines, how to format a value for this area.                                                                                                                                                                                   | -         |
| default_factory | `DefaultFactory` | true     | The [default factory](#default-factory) that defines how to generate the default target for this area. The factory MUST produce a value that is equal to a value on the board. The cell where the values match, is supposed to be left empty when solving the puzzle. | -         |

Example:

```json
{
    "name": "Day",
    "formatter": {
        "type": "Nth"
    },
    "default_factory": {
        "type": "CurrentDay"
    }
}
```

#### Area Formatter

The area formatter takes the raw value from the board and formats it for display to the user.

##### Plain

Returns the value as is.

```json
{
    "type": "PLain"
}
```

##### Nth

Appends "st", "nd", "rd" or "th" to the value based on its last character.
It is expected that the value ends with a digit.

```json
{
    "type": "Nth"
}
```

##### PrefixSuffix

Appends a prefix and/or suffix to the value.

```json
{
    "type": "PrefixSuffix",
    "prefix": "x",
    "suffix": "z"
}
```

#### Default Factory

The default factory generates the default target for an area.
The product of the default factory MUST be equal to a value on the board in the area the factory is used for.

##### Fixed

Returns a fixed value.

```json
{
    "type": "Fixed",
    "value": "example"
}
```

##### CurrentDay

Returns the current day of the month as a string.

```json
{
    "type": "CurrentDay"
}
```

##### CurrentMonthShort

Returns the current month as a three-letter abbreviation.
E.g., "Jan" for January, "Feb" for February, etc.

```json
{
    "type": "CurrentMonthShort"
}
```

##### CurrentYear2FirstDigit

Returns the first digit of the current 2-digit-year as a string.
E.g., "2" for the year 26.

```json
{
    "type": "CurrentYear2FirstDigit"
}
```

##### CurrentYear2SecondDigit

Returns the second digit of the current 2-digit-year as a string.
E.g., "6" for the year 26.

```json
{
    "type": "CurrentYear2SecondDigit"
}
```

extern crate piston_window;

const LETTERS: [[u8;5];26] = [
    //A
    [0b11111,
     0b10001,
     0b11111,
     0b10001,
     0b10001,],
    //B
    [0b11110,
     0b10001,
     0b11110,
     0b10001,
     0b11110,],
     //C
    [0b11111,
     0b10000,
     0b10000,
     0b10000,
     0b11111,],
     //D
    [0b11110,
     0b10001,
     0b00001,
     0b10001,
     0b11110,],
     //E
    [0b11111,
     0b10000,
     0b11110,
     0b10000,
     0b11111,],
     //F
    [0b11111,
     0b10000,
     0b11110,
     0b10000,
     0b10000,],
     //G
    [0b11111,
     0b10000,
     0b10011,
     0b10001,
     0b11111],
     //H
    [0b10001,
     0b10001,
     0b11111,
     0b10001,
     0b10001],
     //I
    [0b00100,
     0b00100,
     0b00100,
     0b00100,
     0b00100],
     //J
     [
        0b01111,
        0b00001,
        0b00001,
        0b00001,
        0b11111
    ],
    //K
      [
        0b10001,
        0b10010,
        0b10100,
        0b10010,
        0b10001,
      ],
      //L
      [
          0b10000,
          0b10000,
          0b10000,
          0b10000,
          0b11111
      ],
      //M
      [
          0b11111,
          0b10101,
          0b10101,
          0b10101,
          0b10001,
      ],
      //N
      [
          0b10001,
          0b11001,
          0b10101,
          0b10011,
          0b10001
      ],
      //O
      [
          0b01110,
          0b10001,
          0b10001,
          0b10001,
          0b01110,
      ],
      //P
      [
          0b11111,
          0b10001,
          0b11111,
          0b10000,
          0b10000,
      ],
      //Q
      [
          0b11111,
          0b10001,
          0b10001,
          0b11111,
          0b00001,
      ],
      //R
      [
          0b11111,
          0b10001,
          0b11111,
          0b10010,
          0b10001,
      ],
      //S
      [
          0b11111,
          0b10000,
          0b11111,
          0b00001,
          0b11111,
      ],
      [
          0b11111,
          0b00100,
          0b00100,
          0b00100,
          0b00100,
      ],
      [
          0b10001,
          0b10001,
          0b10001,
          0b10001,
          0b01110,
      ],
      [
          0b10001,
          0b10001,
          0b01010,
          0b01010,
          0b00100,
      ],
      [
          0b10001,
          0b10101,
          0b10101,
          0b10101,
          0b11111
      ],
      [
          0b10001,
          0b01010,
          0b00100,
          0b01010,
          0b10001
      ],
      [
          0b10001,
          0b10001,
          0b11111,
          0b00100,
          0b00100,
      ],
      [
          0b11111,
          0b00010,
          0b00100,
          0b01000,
          0b11111
      ],
];

const NUMBERS: [[u8;5];10] = [
    [
        0b11111,
        0b10011,
        0b10101,
        0b11001,
        0b11111,
    ],
    [
        0b01100,
        0b10100,
        0b00100,
        0b00100,
        0b00100,
    ],
    [
        0b11111,
        0b00001,
        0b01110,
        0b10000,
        0b11111,
    ],
    [
        0b11111,
        0b00001,
        0b01111,
        0b00001,
        0b11111,
    ],
    [
        0b10001,
        0b10001,
        0b11111,
        0b00001,
        0b00001,
    ],
        [
        0b11111,
        0b10000,
        0b11111,
        0b00001,
        0b11111,
    ],
        [
        0b11111,
        0b10000,
        0b11111,
        0b10001,
        0b11111,
    ],
            [
        0b11111,
        0b00011,
        0b00100,
        0b01000,
        0b10000,
    ],
            [
        0b11111,
        0b10001,
        0b11111,
        0b10001,
        0b11111,
    ],
            [
        0b11111,
        0b10001,
        0b11111,
        0b00001,
        0b11111,
    ],
];

const SIGNS: [[u8;5];9] = [
    //+
    [
        0b00100,
        0b00100,
        0b11111,
        0b00100,
        0b00100,
    ],
    //-
    [
        0b00000,
        0b00000,
        0b11111,
        0b00000,
        0b00000,
    ],
    //times *
        [
        0b00000,
        0b01010,
        0b00100,
        0b01010,
        0b00000,
    ],
    //division /
        [
        0b00100,
        0b00000,
        0b11111,
        0b00000,
        0b00100,
    ],
    //dot .
        [
        0b00000,
        0b00000,
        0b00000,
        0b00000,
        0b00100,
    ],
    //:
        [
        0b00100,
        0b00000,
        0b00000,
        0b00000,
        0b00100,
    ],
    // exclaim
            [
        0b00100,
        0b00100,
        0b00100,
        0b00000,
        0b00100,
    ],
    //?
            [
        0b11110,
        0b10001,
        0b00110,
        0,
        0b00100,
    ],
    //=
    [
        0,
        0b11111,
        0,
        0b11111,
        0,
    ]  
];

pub fn drawstring(thestr: &str, x: f64, y:f64, scalarx: f64, scalary: f64, color: [f32;4], transform: [[f64;3];2], g: &mut piston_window::G2d){
    let oldx = x;
    let mut x = x;
    let mut y = y;
    for charackter in thestr.chars(){
        match charackter{
            // >>> for i in range(ord('A'), ord('Z')+1):
            //...     print("'%c'=>{drawletter(&LETTERS[%i], x, y, scalarx, scalary, color, transform, g);},"%(chr(i),i-ord('A')))
            'A'=>{drawletter(&LETTERS[0], x, y, scalarx, scalary, color, transform, g);},
            'B'=>{drawletter(&LETTERS[1], x, y, scalarx, scalary, color, transform, g);},
            'C'=>{drawletter(&LETTERS[2], x, y, scalarx, scalary, color, transform, g);},
            'D'=>{drawletter(&LETTERS[3], x, y, scalarx, scalary, color, transform, g);},
            'E'=>{drawletter(&LETTERS[4], x, y, scalarx, scalary, color, transform, g);},
            'F'=>{drawletter(&LETTERS[5], x, y, scalarx, scalary, color, transform, g);},
            'G'=>{drawletter(&LETTERS[6], x, y, scalarx, scalary, color, transform, g);},
            'H'=>{drawletter(&LETTERS[7], x, y, scalarx, scalary, color, transform, g);},
            'I'=>{drawletter(&LETTERS[8], x, y, scalarx, scalary, color, transform, g);},
            'J'=>{drawletter(&LETTERS[9], x, y, scalarx, scalary, color, transform, g);},
            'K'=>{drawletter(&LETTERS[10], x, y, scalarx, scalary, color, transform, g);},
            'L'=>{drawletter(&LETTERS[11], x, y, scalarx, scalary, color, transform, g);},
            'M'=>{drawletter(&LETTERS[12], x, y, scalarx, scalary, color, transform, g);},
            'N'=>{drawletter(&LETTERS[13], x, y, scalarx, scalary, color, transform, g);},
            'O'=>{drawletter(&LETTERS[14], x, y, scalarx, scalary, color, transform, g);},
            'P'=>{drawletter(&LETTERS[15], x, y, scalarx, scalary, color, transform, g);},
            'Q'=>{drawletter(&LETTERS[16], x, y, scalarx, scalary, color, transform, g);},
            'R'=>{drawletter(&LETTERS[17], x, y, scalarx, scalary, color, transform, g);},
            'S'=>{drawletter(&LETTERS[18], x, y, scalarx, scalary, color, transform, g);},
            'T'=>{drawletter(&LETTERS[19], x, y, scalarx, scalary, color, transform, g);},
            'U'=>{drawletter(&LETTERS[20], x, y, scalarx, scalary, color, transform, g);},
            'V'=>{drawletter(&LETTERS[21], x, y, scalarx, scalary, color, transform, g);},
            'W'=>{drawletter(&LETTERS[22], x, y, scalarx, scalary, color, transform, g);},
            'X'=>{drawletter(&LETTERS[23], x, y, scalarx, scalary, color, transform, g);},
            'Y'=>{drawletter(&LETTERS[24], x, y, scalarx, scalary, color, transform, g);},
            'Z'=>{drawletter(&LETTERS[25], x, y, scalarx, scalary, color, transform, g);},
            '0'=>{drawletter(&NUMBERS[0], x, y, scalarx, scalary, color, transform, g);},
            '1'=>{drawletter(&NUMBERS[1], x, y, scalarx, scalary, color, transform, g);},
            '2'=>{drawletter(&NUMBERS[2], x, y, scalarx, scalary, color, transform, g);},
            '3'=>{drawletter(&NUMBERS[3], x, y, scalarx, scalary, color, transform, g);},
            '4'=>{drawletter(&NUMBERS[4], x, y, scalarx, scalary, color, transform, g);},
            '5'=>{drawletter(&NUMBERS[5], x, y, scalarx, scalary, color, transform, g);},
            '6'=>{drawletter(&NUMBERS[6], x, y, scalarx, scalary, color, transform, g);},
            '7'=>{drawletter(&NUMBERS[7], x, y, scalarx, scalary, color, transform, g);},
            '8'=>{drawletter(&NUMBERS[8], x, y, scalarx, scalary, color, transform, g);},
            '9'=>{drawletter(&NUMBERS[9], x, y, scalarx, scalary, color, transform, g);},
            '+'=>{drawletter(&SIGNS[0], x, y, scalarx, scalary, color, transform, g);},
            '-'=>{drawletter(&SIGNS[1], x, y, scalarx, scalary, color, transform, g);},
            '*'=>{drawletter(&SIGNS[2], x, y, scalarx, scalary, color, transform, g);},
            '/'=>{drawletter(&SIGNS[3], x, y, scalarx, scalary, color, transform, g);},
            '.'=>{drawletter(&SIGNS[4], x, y, scalarx, scalary, color, transform, g);},
            ':'=>{drawletter(&SIGNS[5], x, y, scalarx, scalary, color, transform, g);},
            '!'=>{drawletter(&SIGNS[6], x, y, scalarx, scalary, color, transform, g);},
            '?'=>{drawletter(&SIGNS[7], x, y, scalarx, scalary, color, transform, g);},
            '='=>{drawletter(&SIGNS[8], x, y, scalarx, scalary, color, transform, g);},
            //nothing happens for space
            ' '=>{},
            '\n'=>{
                x = oldx;
                y += 6.0*scalary;
                continue;
            },
            //Everything else gets X (like small letters abcd...)
            _ => {drawletter(&LETTERS[23], x, y, scalarx, scalary, color, transform, g);},
        }
        x += 6.0*scalarx;
    }
}

fn drawletter(letter: &[u8;5], x:f64, y:f64, scalarx: f64, scalary: f64, color: [f32;4], transform: [[f64;3];2], g: &mut piston_window::G2d){
    for (index_y, row) in letter.iter().enumerate(){
        for i in (0..5).rev(){
            if (1<<i)&row != 0{
                piston_window::rectangle(color, [x+(4-i) as f64*scalarx, y+index_y as f64*scalary, scalarx, scalary], transform, g);
            }
        }
    }
}
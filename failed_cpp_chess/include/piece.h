#pragma once

#include <cstdint>

using square_t = uint8_t;

enum Piece : square_t{
    EMPTY =  0b0,
    PAWN =   0b1,
    KNIGHT = 0b10,
    BISHOP = 0b100,
    ROOK =   0b1000,
    QUEEN =  0b10000,
    KING =   0b100000,
    P_MASK = 0b111111,
    CHECK_BIT = 0b10000000,
};

enum Color : square_t{
    C_MASK = 0b1000000,
    WHITE =  0b00000000,
    BLACK =  C_MASK,
};
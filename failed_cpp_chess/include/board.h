#pragma once

#include <array>
#include <cstdint>
#include <iostream>
#include <algorithm>
#include "piece.h"

class Board{
public:
constexpr Board() :  board { //is fliped on x and y axis, dont index into with ranks and files!!
        ROOK|BLACK, KNIGHT|BLACK, BISHOP|BLACK, KING|BLACK, QUEEN|BLACK, BISHOP|BLACK, KNIGHT|BLACK, ROOK|BLACK, //8
        PAWN|BLACK, PAWN|BLACK,   PAWN|BLACK,   PAWN|BLACK, PAWN|BLACK,  PAWN|BLACK,   PAWN|BLACK,   PAWN|BLACK, //7
        EMPTY,      EMPTY,        EMPTY,        EMPTY,      EMPTY,       EMPTY,        EMPTY,        EMPTY,      //6
        EMPTY,      EMPTY,        EMPTY,        EMPTY,      EMPTY,       EMPTY,        EMPTY,        EMPTY,      
        EMPTY,      EMPTY,        EMPTY,        EMPTY,      EMPTY,       EMPTY,        EMPTY,        EMPTY,
        EMPTY,      EMPTY,        EMPTY,        EMPTY,      EMPTY,       EMPTY,        EMPTY,        EMPTY,      //3
        PAWN|WHITE, PAWN|WHITE,   PAWN|WHITE,   PAWN|WHITE, PAWN|WHITE,  PAWN|WHITE,   PAWN|WHITE,   PAWN|WHITE, //2
        ROOK|WHITE, KNIGHT|WHITE, BISHOP|WHITE, KING|WHITE, QUEEN|WHITE, BISHOP|WHITE, KNIGHT|WHITE, ROOK|WHITE, //1
} {};     
    square_t set(uint8_t file, uint8_t rank, Piece p, Color Color);
    square_t set(uint8_t file, uint8_t rank, square_t p);
    square_t get(uint8_t file, uint8_t rank) const;
    void run();

    friend struct MoveGen;
private:
    enum{
        MAX_RANK = 8,
        MAX_FILE = 8,
    };
    struct MoveGen{

        uint8_t playerColor;
        uint8_t rank=0, file=0;
        //distance from selected square to impossible move
        std::array<uint8_t, 8> pieceDist = {};
        //a bitmask for weather or not a piece is blocked from moving to a square
        //directions array
        std::array<int8_t, 16> dir;
        //possible moves for a piece to make
        std::array<uint16_t, 15> moveMask;
        
        std::array<uint8_t, 8> bitBoard;

        constexpr MoveGen(square_t p) : playerColor{static_cast<uint8_t>(p & C_MASK)},
         dir {
            0,1, //up
            1,1,  
            1,0, // right
            1,-1,
            0,-1, //down
            -1,-1,
            -1,0, //left
            -1, 1,
        }, bitBoard{0xff,0xff,0xff,0xff,0xff,0xff,0xff,0xff}
        {
            p = p&P_MASK;
            switch(p){
            case PAWN:
            moveMask = {
                0b000000000000000,
                0b000000000000000,
                0b000000000000000,
                0b000000000000000,
                0b000000000000000,
                0b000000010000000,
                0b000000111000000,
                0b000000000000000,
                0b000000000000000,
                0b000000000000000,
                0b000000000000000,  
                0b000000000000000,              
                0b000000000000000,                
                0b000000000000000,
                0b000000000000000,
            };
            break;
            case KNIGHT:
            moveMask = {
                0b000000000000000,
                0b000000000000000,
                0b000000000000000,
                0b000000000000000,
                0b000000000000000,
                0b000000101000000,
                0b000001000100000,
                0b000000000000000,
                0b000001000100000,
                0b000000101000000,
                0b000000000000000,  
                0b000000000000000,              
                0b000000000000000,                
                0b000000000000000,
                0b000000000000000,
            };
            break;
            case BISHOP:
            moveMask = {
                0b100000000000001,
                0b010000000000010,
                0b001000000000100,
                0b000100000001000,
                0b000010000010000,
                0b000001000100000,
                0b000000101000000,
                0b000000000000000,
                0b000000101000000,
                0b000001000100000,
                0b000010000010000,  
                0b000100000001000,              
                0b001000000000100,                
                0b010000000000010,
                0b100000000000001,
            };
            break;
            case ROOK:
            moveMask = {
                0b000000010000000,
                0b000000010000000,
                0b000000010000000,
                0b000000010000000,
                0b000000010000000,
                0b000000010000000,
                0b000000010000000,
                0b111111101111111,
                0b000000010000000,
                0b000000010000000,
                0b000000010000000,  
                0b000000010000000,              
                0b000000010000000,                
                0b000000010000000,
                0b000000010000000,
            };
            break;
            case QUEEN:
            moveMask = {
                0b100000010000001,
                0b010000010000010,
                0b001000010000100,
                0b000100010001000,
                0b000010010010000,
                0b000001010100000,
                0b000000111000000,
                0b111111101111111,
                0b000000111000000,
                0b000001010100000,
                0b000010010010000,  
                0b000100010001000,              
                0b001000010000100,                
                0b010000010000010,
                0b100000010000001,
            };
            break;
            case KING:
            moveMask = {
                0b000000000000000,
                0b000000000000000,
                0b000000000000000,
                0b000000000000000,
                0b000000000000000,
                0b000000000000000,
                0b000000111000000,
                0b000000101000000,
                0b000000111000000,
                0b000000000000000,
                0b000000000000000,  
                0b000000000000000,              
                0b000000000000000,                
                0b000000000000000,
                0b000000000000000,
            };
            break;
            default:
                moveMask = {0,0,0,0,0,0,0,0,0,0,0,0,0,0,0};
            }
            if(playerColor == BLACK){
                std::reverse(moveMask.begin(), moveMask.end());
            }
        }   
        //ray cast to see where a piece can move
        void generatePieceDist(const Board& b, uint8_t file, uint8_t rank);
        //fill bitBoard with usefull bits
        void generateBoard();
        //check if spot is true in bitBoard
        bool isValid(uint8_t file, uint8_t rank) const;

    };

    //MoveGen& getMoveGenForPeice(square_t piece);
    char getPieceChar(square_t s) const;
    square_t createPiece(Piece p, Color c) const;

    static uint8_t _toX(uint8_t file){return MAX_FILE - file;}
    static uint8_t _toY(uint8_t rank){return MAX_RANK - rank;}

    uint8_t getArrPos(uint8_t rank, uint8_t file) const;

    uint8_t convertFileToNum(char file) const;
    uint8_t convertRankToNum(char rank) const;

    char convertNumToFile(uint8_t file) const;
    char convertNumToRank(uint8_t rank) const;

    bool makeMove(const MoveGen& g, uint8_t ofile, uint8_t orank, uint8_t nfile, uint8_t nrank);


    void printSquare(square_t s, bool isBlack, bool hightlight) const;
    void print(uint8_t file, uint8_t rank, const MoveGen& G) const;
    void print() const;

    std::pair<uint8_t,uint8_t> getInput();
    bool validInput(char file, char rank);

    void takeHumanTurn();


    int activePlayer = 0;

    uint32_t moveNumber = 1;

    //std::array<MoveGen,6> whiteMoveGen;
    //std::array<MoveGen,6> blackMoveGen;

    std::array<square_t, 64> board;


};

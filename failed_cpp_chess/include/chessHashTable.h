#pragma once

#include <cstdint>
#include <array>
#include "piece.h"

class ChessHashTable{
    square_t type;
    uint8_t file, rank;
    uint8_t iter;
    std::array<uint8_t, 4> bitTable;
    static const std::array<std::pair<int8_t,int8_t>, 4> dirs;

public:
    constexpr ChessHashTable(){};
    constexpr ChessHashTable(square_t p, uint8_t file, uint8_t rank) 
        : type{p}, file{file}, rank{rank} {};
    bool isEmpty() const {
        bool e = true;
        for(int i = 0; i < bitTable.size(); ++i){
            e = e & !bitTable[i];
        }
        return e;
    }
    uint8_t size() const{
        int res = 0;
        for(int i = 0; i < bitTable.size(); ++i){
            uint32_t bits = bitTable[i];
            //dear compiler, pls replace with popcount if thats an instruction. from, sam
            while(bits){
                bits &= bits - 1;
                res += 1;
            }
        }
        
    }
    int _index(int df, int dr)const{
        return  ((dr & 0x80000000) == (df & 0x80000000)) * 3 + (dr == 0) * 2 + (df == 0) * 1;
    }

    void set(uint8_t f, uint8_t r){
        int df = file - f;
        int dr = rank - r;
        int index = _index(df, dr);
        int shift = (dr==0) * f + (df==0) * r + (index == 3) * abs(r+f) + (index == 0) * abs(r-f);
        bitTable[index] |= 1 << shift;
    }
    void unset(uint8_t f, uint8_t r){
        int df = file - f;
        int dr = rank - r;
        int index = _index(df, dr);
        int shift = (dr==0) * f + (df==0) * r + (index == 3) * abs(r+f) + (index == 0) * abs(r-f);
        bitTable[index] &= ~(1 << shift);
    }
    bool get(uint8_t f, uint8_t r) const {
        int df = file - f;
        int dr = rank - r;
        int index = _index(df, dr);
        int shift = (dr==0) * f + (df==0) * r + (index == 3) * abs(r+f)/2 + (index == 0) * abs(r-f)/2;
        return bitTable[index] & (1 << shift);
    }
    void start(){
        iter = 0;
    }
    bool end(){
        return iter > 32;
    }
    bool next(){
        
    }
    std::pair<uint8_t, uint8_t> get(){
        auto dir = dirs[iter/8];
        uint8_t sr = (dir.second >= 0) * 1 + (dir.second < 0) * 8;
        uint8_t f = dir.first * (iter % 8) + 1;
        uint8_t r = dir.second * (iter % 8) + sr;
        return std::make_pair(f,r);
    }
};

const std::array<std::pair<int8_t,int8_t>,4> ChessHashTable::dirs = { 
    { {1,1}, {0,1}, {1,0}, {1,-1} }
};
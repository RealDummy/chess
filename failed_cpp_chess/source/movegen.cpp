#include "board.h"

void Board::MoveGen::generateBoard(){
        for (int i = 0; i < pieceDist.size(); ++i){
            int df = dir[i*2];
            int dr = dir[i*2 + 1];
            int f = df * pieceDist[i] + file + df;
            int r = dr * pieceDist[i] + rank + dr;
            while(f > 0 && f <= Board::MAX_FILE && r > 0 && r <= Board::MAX_RANK){
                bitBoard[_toY(r)] &= ~(0b1 << (_toX(f)));
                f += df;
                r += dr;
            }
        }
        if(playerColor == WHITE){
            for(int i = 0; i < Board::MAX_RANK; ++i){
                bitBoard[i] &= static_cast<uint8_t>( moveMask[rank + i - 1] >> (file - 1));
            }
        }
        else{
            for(int i = 0; i < Board::MAX_RANK; ++i){
                bitBoard[i] &= static_cast<uint8_t>( moveMask[rank + i -1] >> (file - 1));
            }
        }

    }
void Board::MoveGen::generatePieceDist(const Board& b, uint8_t nfile, uint8_t nrank){
    file = nfile;
    rank = nrank;
    for(int i = 0; i < dir.size(); i += 2){
        uint8_t f=nfile,r=nrank, c=0;
        while(f > 0 && f <= Board::MAX_FILE && r > 0 && r <= Board::MAX_RANK){
            if( c > 0 && (b.get(f,r) & P_MASK) != EMPTY) {
                c += (b.get(file,rank) & C_MASK) != (b.get(f,r) & C_MASK);
                break;
            };
            c += 1;
            f += dir[i];
            r += dir[i+1];
        }
        pieceDist[i/2] = c;
    }        
}
bool Board::MoveGen::isValid(uint8_t file, uint8_t rank) const{
    return bitBoard[_toY(rank)] & (0x1 << (_toX(file)));
}
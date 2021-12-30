#include "board.h"
#include <iostream>

/* Board::MoveGen& Board::getMoveGenForPeice(square_t piece){
    square_t p = piece & P_MASK;
    int bitSet = 0;
    while(p != PAWN){
        p = p >> 1;
        bitSet += 1;
    }
    if( (piece & C_MASK) == WHITE){
        return whiteMoveGen[bitSet];
    }
    else{
        return blackMoveGen[bitSet];
    }
}
*/

uint8_t Board::getArrPos(uint8_t file, uint8_t rank) const {
    return _toY(rank) * MAX_FILE + _toX(file);
}

square_t Board::createPiece(Piece p, Color c) const{
    return p | c;
}

square_t Board::set(uint8_t file, uint8_t rank, Piece p, Color c){
    return set(file,rank, createPiece(p,c));
}
square_t Board::get(uint8_t file, uint8_t rank) const{
    return board[getArrPos(file,rank)];
}
square_t Board::set(uint8_t file, uint8_t rank, square_t p){
    square_t res = board[getArrPos(file,rank)];
    board[getArrPos(file,rank)] = p;
    return res;
}

char Board::getPieceChar(square_t s) const{
    char res;
    switch(s & P_MASK){
        case EMPTY:
            res =  ' ';
            break;
        case PAWN:
            res = 'P';
            break;
        case KNIGHT:
            res = 'N';
            break;
        case BISHOP:
            res = 'B';
            break;
        case ROOK:
            res = 'R';
            break;
        case QUEEN:
            res = 'Q';
            break;
        case KING:
            res = 'K';
            break;
    }
    return res;
}

void Board::printSquare(square_t s, bool isBlack, bool highlight) const{
    //bakcground color is 40 if isBlack, else 47. if hightlight, background color = 43
    std::cout << "\033[" << '4' << static_cast<char>('0' + (!highlight * (!(isBlack) * 7) + highlight * 3) ) << ';'
    //text color is blue for white, red for black
        << (((s & C_MASK ) == WHITE) ?  "94m " : "91m ")
        << getPieceChar(s) 
        << " \033[0m";
}
void Board::print() const{
    for(uint8_t i = MAX_RANK; i > 0; --i){
        for(uint8_t j = 1; j <= MAX_FILE; ++j){
            printSquare(board[getArrPos(j,i)], !(i % 2) ^ (j % 2), false );
        }
        std::cout << ' ' <<static_cast<char>('0' +  i) << "\n";
        
    }
    std::cout << " A  B  C  D  E  F  G  H\n";
    putc('\n', stdout);
    putc('\n', stdout);
}

void Board::print(uint8_t file, uint8_t rank, const MoveGen& G) const{
    for(uint8_t i = MAX_RANK; i > 0; --i){
        for(uint8_t j = 1; j <= MAX_FILE; ++j){
            printSquare(
                board[getArrPos(j,i)], 
                !(i % 2) ^ (j % 2), 
                (i == rank && j == file) || G.isValid(j,i)
            );
        }
        std::cout << ' ' <<static_cast<char>('0' +  i) << "\n";
        
    }
    std::cout << " A  B  C  D  E  F  G  H\n";
    putc('\n', stdout);
    putc('\n', stdout);
}

uint8_t Board::convertFileToNum(char file) const{
    bool isUpper = file <= 'Z';
    return isUpper * (file - 64) + !isUpper * (file - 96); //'A' and 'a' - 1
}
uint8_t Board::convertRankToNum(char rank) const{
    return rank - '0';
}
char Board::convertNumToFile(uint8_t file) const{
    return file + 64; //'A' - 1
}
char Board::convertNumToRank(uint8_t rank) const{
    return rank + '0';
}

bool Board::makeMove(const MoveGen& G, uint8_t ofile, uint8_t orank, uint8_t nfile, uint8_t nrank){
    square_t p = get(ofile,orank);
    //insert move logic here
    if(p == EMPTY || !G.isValid(nfile,nrank)){
        return false;
    }
    set(ofile, orank, EMPTY);
    set(nfile, nrank,p);
    //insert capture logic here
    moveNumber += 1;
    activePlayer = activePlayer ^ 1;
    return true;
}
bool Board::validInput(char file, char rank){
    bool fileOK = ((file >= 'a') && (file <= 'h')) || ((file >= 'A') && (file <= 'H'));
    bool rankOK = rank >= '1' && rank <= '8';
    return fileOK && rankOK;
}

void Board::run(){
    while(true){ //change to a running var later
        takeHumanTurn();
    }
}

void Board::takeHumanTurn(){
    system("clear");
    print();
    
    std::cout << "Select Square [a-f][1-8]: ";
    auto moveFrom = getInput();

    MoveGen G(get(moveFrom.first, moveFrom.second));
    G.generatePieceDist(*this, moveFrom.first, moveFrom.second);
    G.generateBoard();
    
    while(false){//!isLegal(G,moveFrom.first, moveFrom.second)){
        std::cout << convertNumToFile(moveFrom.first) << convertNumToRank(moveFrom.second)
            << " has no legal moves.\n";
        std::cout << "Select Square [a-f][1-8]: ";
        moveFrom = getInput();
        G = MoveGen(get(moveFrom.first, moveFrom.second));
    }

    print(moveFrom.first, moveFrom.second, G);

    std::cout << "Make Move [a-f][1-8]: ";
    auto moveTo = getInput();

    while(!makeMove(G, moveFrom.first, moveFrom.second, moveTo.first, moveTo.second)){
        std::cout << "Make a valid Move: ";
        moveTo = getInput();
    }
}



std::pair<uint8_t, uint8_t> Board::getInput(){
    char file, rank;
    do{
        std::cin >> file >> rank;
    }while(!validInput(file, rank));
    return {convertFileToNum(file), convertRankToNum(rank)};
}



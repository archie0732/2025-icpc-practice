#include <bits/stdc++.h>

using namespace std;


bool checkIsRepeat(int year){
  set<int> st;

  while(year > 0){
    int digit = year % 10;
    if(st.count(digit)) return true;
    st.insert(digit);
    year /= 10;
  }

  return false;
}

int main(){
  int year;
  cin >> year;
  while(true){
    year++;
    if(!checkIsRepeat(year)){
      cout << year << endl;
      break;
    } 
  }
}


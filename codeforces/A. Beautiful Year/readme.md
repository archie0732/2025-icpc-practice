# Beautiful Year 

>[!note]
> - `800`


## 題目

輸入一個年，找出下一個數字不重複的年

## 解題

too ez, skip


### CPP CODE

```cpp
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

```




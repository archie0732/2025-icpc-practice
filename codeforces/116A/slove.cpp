#include <bits/stdc++.h>

using namespace std;

int main(){

  int cur = 0, mx = 0;

  int n;
  cin>>n;

  while(n--){
    int up, down;
    cin >> down >> up;

    cur = cur + up - down;
    //cout << "cur: " << cur << endl;
    mx = max(mx, cur);
  }
  cout << mx << endl;
}
#include<bits/stdc++.h>

using namespace std;


struct Point{
  int x, y;
};


bool isIntersect(Point a, Point b, Point c, Point d){
  long long d1 = cross(p1, p2, q1);
    long long d2 = cross(p1, p2, q2);
    long long d3 = cross(q1, q2, p1);
    long long d4 = cross(q1, q2, p2);

    if (((d1 > 0 && d2 < 0) || (d1 < 0 && d2 > 0)) &&
        ((d3 > 0 && d4 < 0) || (d3 < 0 && d4 > 0))) {
        return true; // 一般情況：兩線段相交
    }

    // 特殊情況：共線，判斷是否在線段上
    if (d1 == 0 && onSegment(p1, p2, q1)) return true;
    if (d2 == 0 && onSegment(p1, p2, q2)) return true;
    if (d3 == 0 && onSegment(q1, q2, p1)) return true;
    if (d4 == 0 && onSegment(q1, q2, p2)) return true;

    return false;
}

int cross(Point a, Point b, Point c){
  return (b.x - a.x) * (c.y - a.y) - (b.y - a.y) * (c.x - a.x);
}

bool isOnSegment(Point a, Point b, Point c){
  return min(a.x, b.x) <= c.x && c.x <= max(a.x, b.x) && min(a.y, b.y) <= c.y && c.y <= max(a.y, b.y);
}


int main(){

  Point a = {0, 0};
  Point b = {1, 1};
  Point c = {2, 2};
  Point d = {3, 3};
  
  cout << isIntersect(a, b, c, d) << endl;


}
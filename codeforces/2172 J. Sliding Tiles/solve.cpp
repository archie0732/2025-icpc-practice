#include <bits/stdc++.h>

using namespace std;

void print(vector<vector<int>> board)
{
    for (auto i : board)
    {
        for (auto a : i)
        {
            cout << a << " ";
        }
        cout << endl;
    }
}

int main()
{

    int n;
    cin >> n;

    vector<vector<int>> board(n, vector<int>(n, 0));
    vector<int> walls(n - 1, 0);
    for (int i = 0; i < n; i++)
    {
        int a;
        cin >> a;
        for (int j = 0; j < a; j++)
        {
            board[i][j] = 1;
        }
    }

    // print(board);

    for (int i = 0; i < n - 1; i++)
    {
        int a;
        cin >> a;
        walls[i] = a;
    }

    for (int i = n - 1; i >= 0; i--)
    {
        for (int j = 0; j < n - 1; j++)
        {
            if (i > walls[j] - 1 && board[j][i] > 0)
            {
                board[j + 1][i] += board[j][i];
                board[j][i] = 0;
            }
        }
    }
    vector<int> ans(n, 0);
    // print(board);

    for (int i = 0; i < n; i++)
    {
        for (auto j : board[i])
        {
            for (int k = 0; k < j; k++)
            {
                ans[i - k]++;
            }
        }
    }

    for (auto i : ans)
    {
        cout << i << " ";
    }
    cout << '\n';

    return 0;
}

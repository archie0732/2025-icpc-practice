# VSCode C/C++ 環境設定

## 安裝

### 安裝 gcc / g++

gcc 和 g++ 是撰寫 C 或 C++ 程式時必要的編譯器， Windows 在這裡推薦使用 MSYS2 來安裝。

:::warning
:warning: **注意** —— 以下內容僅限於 Windows 系統。
:::

1. 到 [MSYS2 官網](https://www.msys2.org/)下載 MSYS2 安裝器。
   ![](https://hackmd.io/_uploads/rynqprC-T.png)
   
2. 開啟 MSYS2 安裝器。
   ![](https://hackmd.io/_uploads/HJ0C2D0Za.png)
   
3. 設定 MSYS2 安裝路徑。
   ![](https://hackmd.io/_uploads/Hk646vRZa.png)
   
4. 設定 MSYS2 開始選單圖示。
   ![](https://hackmd.io/_uploads/Syl_6wAWa.png)
   
5. 安裝完成後開啟 MSYS2。
 
6. 輸入 `pacman -S --needed base-devel mingw-w64-ucrt-x86_64-toolchain` 安裝 C/C++ 開發工具鏈。
   ![image](https://hackmd.io/_uploads/Hyyn4y660.png)

7. 接著按 `Enter` 開始安裝工具。
   ![image](https://hackmd.io/_uploads/BJWlHJ66A.png)

8. 將工具加入 PATH 環境變數中。
   1. 在搜尋中找到「編輯系統環境變數」。
      ![](https://hackmd.io/_uploads/SJAiKw0Wp.png)
   2. 點擊「環境變數」。
      ![](https://hackmd.io/_uploads/ByMHmvR-a.png)
   3. 在「系統變數」部分，選擇「Path」並點擊編輯。
      ![](https://hackmd.io/_uploads/Bk0zNvCW6.png)
   4. 將 MSYS2 安裝位置新增到 PATH 中（預設位置：`C:\msys64\ucrt64\bin`）
     ![image](https://hackmd.io/_uploads/BJv5W1aaC.png)
   5. 點擊確定
    
9. 打開一個新的終端機視窗，然後輸入以下指令，確保工具皆已成功安裝：
    ```bash
    gcc --version & g++ --version & gdb --version
    ```
    ![image](https://hackmd.io/_uploads/H11wIJppC.png)

10. 大功告成！

### 安裝編輯器

我們推薦使用 [Microsoft Visual Studio Code](https://code.visualstudio.com) 作為主要的程式碼編輯器，簡稱 VSCode。

安裝 VSCode 請到[下載頁面](https://code.visualstudio.com/Download) 選取你的作業系統的版本進行安裝。

### 安裝延伸模組

開啟 VSCode 後點擊左側的延伸模組分頁，搜尋 C/C++ Extension Pack，找到後點擊安裝即可。

![image](https://hackmd.io/_uploads/HJu0UyTa0.png)

### 其他推薦安裝的延伸模組

這裡有列出了一些非常推薦安裝的延伸模組，可提升程式碼編寫體驗：

- [**Chinese (Traditional) Language Pack for Visual Studio Code**](https://marketplace.visualstudio.com/items?itemName=MS-CEINTL.vscode-language-pack-zh-hant) - VSCode 的繁體中文語言包。
- [**Error Lens**](https://marketplace.visualstudio.com/items?itemName=usernamehw.errorlens) - 讓錯誤訊息直接顯示在行內，並用明顯的背景顏色提示。
- [**C/C++ Snippets**](https://marketplace.visualstudio.com/items?itemName=hars.CppSnippets) - 讓你使用一些常見的 C/C++ 程式碼片段，可加速編程速度。
- [**Bracket Highlighter**](https://marketplace.visualstudio.com/items?itemName=Durzn.brackethighlighter) - 把游標所在區塊的相對應左右括弧標記起來，並可以讓區塊外的代碼變淡，讓代碼更容易閱讀。
- [Bracket Lens](https://marketplace.visualstudio.com/items?itemName=wraith13.bracket-lens) - 在右括弧的後面寫這個區塊的開頭是什麼，可以讓你在很長的區塊中不用拉到上面玩捉迷藏。
- [Better C++ Syntax](https://marketplace.visualstudio.com/items?itemName=jeff-hykin.better-cpp-syntax) - 更好的語法上色方式。
- [Competitive Programming Helper (cph)](https://marketplace.visualstudio.com/items?itemName=DivyanshuAgrawal.competitive-programming-helper) - 讓你可以輸入測資一次測試程式，不用再一個一個執行手動輸入。
- [TabOut](https://marketplace.visualstudio.com/items?itemName=albert.TabOut) - 按 Tab 離開括號。

### VSCode 自動排版設定

在 VSCode 中，點擊左下角的齒輪圖示以打開設定（或使用 `Ctrl` + `,` 快捷鍵）。

寫程式碼的時候建議將「自動儲存」設為「onFocusChange（當焦點變更時）」。
![](https://hackmd.io/_uploads/r1nhWw0ZT.png)

並將「在儲存時格式化檔案」打勾。
![](https://hackmd.io/_uploads/HyRQfPRZp.png)

這樣在儲存程式碼的時候就會自動排版了。

## 常見問題

### 按下執行後我該選哪個

選路徑有 `msys64` 的選項。

![image](https://hackmd.io/_uploads/H1U7vyTpA.png)

### launch: 'path' does not exist

![](https://hackmd.io/_uploads/rkDMXB0ba.png)

這個錯誤最常見的成因是路徑中含有非 ASCII 字元。

請在執行前先確認檔案路徑中是否含有非 ASCII 字元，有的話把檔案重新命名即可。

### undefined reference to 'WinMain'

把你的原始檔移到 OneDrive **以外**的地方再執行一次

### 終端機無法輸入

執行程式後在 VSCode 的整合終端機無法輸入最常見的成因是 gdb 在安裝時沒有自動設定參數，改使用 MSYS 安裝 gcc / g++ 即可解決。

透過 MSYS2 安裝請詳見 [#安裝 gcc / g++](#安裝-gcc--g)
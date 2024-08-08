# Raspberry Pi Zero 2 プロジェクト

## 概要

このプロジェクトでは、Raspberry Pi Zero 2と以下のコンポーネントを使用して、さまざまなエレクトロニクスプロジェクトをRustで実装します。

- 3色フルカラー SNDLEDモジュール
- マイク/サウンドセンサー
- ホール効果センサーモジュール
- HC06 Bluetoothモジュール

## 接続ガイド

### 1. LED ストロボライト

**コンポーネント:**
- 3色フルカラー SNDLEDモジュール

**接続:**
- **LEDモジュールのVCCピン** → **Raspberry Piの5Vピン**
- **LEDモジュールのGNDピン** → **Raspberry PiのGNDピン**
- **LEDモジュールのデータピン** → **Raspberry PiのGPIO 18（ピン12）**

### 2. 音に反応するLEDライト

**コンポーネント:**
- マイク/サウンドセンサー
- 3色フルカラー SNDLEDモジュール

**接続:**
- **マイクのVCCピン** → **Raspberry Piの3.3Vピン**
- **マイクのGNDピン** → **Raspberry PiのGNDピン**
- **マイクのOUTピン** → **Raspberry PiのGPIO 17（ピン11）**
- **LEDモジュールのVCCピン** → **Raspberry Piの5Vピン**
- **LEDモジュールのGNDピン** → **Raspberry PiのGNDピン**
- **LEDモジュールのデータピン** → **Raspberry PiのGPIO 18（ピン12）**

### 3. 環境音レベルモニタ

**コンポーネント:**
- マイク/サウンドセンサー
- HC06 Bluetoothモジュール

**接続:**
- **マイクのVCCピン** → **Raspberry Piの3.3Vピン**
- **マイクのGNDピン** → **Raspberry PiのGNDピン**
- **マイクのOUTピン** → **Raspberry PiのGPIO 17（ピン11）**
- **HC06のVCCピン** → **Raspberry Piの5Vピン**
- **HC06のGNDピン** → **Raspberry PiのGNDピン**
- **HC06のTXピン** → **Raspberry PiのGPIO 15（ピン10）**
- **HC06のRXピン** → **Raspberry PiのGPIO 14（ピン8）**

### 4. 物体検出システム

**コンポーネント:**
- ホール効果センサーモジュール
- 3色フルカラー SNDLEDモジュール

**接続:**
- **ホールセンサーのVCCピン** → **Raspberry Piの3.3Vピン**
- **ホールセンサーのGNDピン** → **Raspberry PiのGNDピン**
- **ホールセンサーのOUTピン** → **Raspberry PiのGPIO 22（ピン15）**
- **LEDモジュールのVCCピン** → **Raspberry Piの5Vピン**
- **LEDモジュールのGNDピン** → **Raspberry PiのGNDピン**
- **LEDモジュールのデータピン** → **Raspberry PiのGPIO 18（ピン12）**

### 5. BluetoothリモートコントロールLED

**コンポーネント:**
- HC06 Bluetoothモジュール
- 3色フルカラー SNDLEDモジュール

**接続:**
- **HC06のVCCピン** → **Raspberry Piの5Vピン**
- **HC06のGNDピン** → **Raspberry PiのGNDピン**
- **HC06のTXピン** → **Raspberry PiのGPIO 15（ピン10）**
- **HC06のRXピン** → **Raspberry PiのGPIO 14（ピン8）**
- **LEDモジュールのVCCピン** → **Raspberry Piの5Vピン**
- **LEDモジュールのGNDピン** → **Raspberry PiのGNDピン**
- **LEDモジュールのデータピン** → **Raspberry PiのGPIO 18（ピン12）**

### 6. 音響センサーによるLEDパターン生成

**コンポーネント:**
- マイク/サウンドセンサー
- 3色フルカラー SNDLEDモジュール

**接続:**
- **マイクのVCCピン** → **Raspberry Piの3.3Vピン**
- **マイクのGNDピン** → **Raspberry PiのGNDピン**
- **マイクのOUTピン** → **Raspberry PiのGPIO 17（ピン11）**
- **LEDモジュールのVCCピン** → **Raspberry Piの5Vピン**
- **LEDモジュールのGNDピン** → **Raspberry PiのGNDピン**
- **LEDモジュールのデータピン** → **Raspberry PiのGPIO 18（ピン12）**

### 7. Bluetoothでの音量通知システム

**コンポーネント:**
- マイク/サウンドセンサー
- HC06 Bluetoothモジュール

**接続:**
- **マイクのVCCピン** → **Raspberry Piの3.3Vピン**
- **マイクのGNDピン** → **Raspberry PiのGNDピン**
- **マイクのOUTピン** → **Raspberry PiのGPIO 17（ピン11）**
- **HC06のVCCピン** → **Raspberry Piの5Vピン**
- **HC06のGNDピン** → **Raspberry PiのGNDピン**
- **HC06のTXピン** → **Raspberry PiのGPIO 15（ピン10）**
- **HC06のRXピン** → **Raspberry PiのGPIO 14（ピン8）**

### 8. LEDビジュアルアラームシステム

**コンポーネント:**
- ホール効果センサーモジュール
- 3色フルカラー SNDLEDモジュール

**接続:**
- **ホールセンサーのVCCピン** → **Raspberry Piの3.3Vピン**
- **ホールセンサーのGNDピン** → **Raspberry PiのGNDピン**
- **ホールセンサーのOUTピン** → **Raspberry PiのGPIO 22（ピン15）**
- **LEDモジュールのVCCピン** → **Raspberry Piの5Vピン**
- **LEDモジュールのGNDピン** → **Raspberry PiのGNDピン**
- **LEDモジュールのデータピン** → **Raspberry PiのGPIO 18（ピン12）**

### 9. 環境音分析ツール

**コンポーネント:**
- マイク/サウンドセンサー
- 3色フルカラー SNDLEDモジュール

**接続:**
- **マイクのVCCピン** → **Raspberry Piの3.3Vピン**
- **マイクのGNDピン** → **Raspberry PiのGNDピン**
- **マイクのOUTピン** → **Raspberry PiのGPIO 17（ピン11）**
- **LEDモジュールのVCCピン** → **Raspberry Piの5Vピン**
- **LEDモジュールのGNDピン** → **Raspberry PiのGNDピン**
- **LEDモジュールのデータピン** → **Raspberry PiのGPIO 18（ピン12）**

### 10. リアルタイム音響データストリーミング

**コンポーネント:**
- マイク/サウンドセンサー
- HC06 Bluetoothモジュール

**接続:**
- **マイクのVCCピン** → **Raspberry Piの3.3Vピン**
- **マイクのGNDピン** → **Raspberry PiのGNDピン**
- **マイクのOUTピン** → **Raspberry PiのGPIO 17（ピン11）**
- **HC06のVCCピン** → **Raspberry Piの5Vピン**
- **HC06のGNDピン** → **Raspberry

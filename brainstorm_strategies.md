# **My collection of thoughts and ideas on different kinds of new strategies in the DeFi space.**

## _Legenda_:

- **_buy_** = buy order by the bot
- **_sell_** = sell order by the bot
- **_Vbuy_** = buy order by a victim
- **_Vsell_** = sell order by a victim
- **_Lsell_** = large sell order by the bot
- **_Lbuy_** = large buy order by the bot

## **Table of Contents**

1. [New type of sandwhich ops](#new-type-of-sandwhich-ops)

- 1.1 [Deeper explaination of "New type of sandwhich ops"?](#deeper-explaination-of-new-type-of-sandwhich-ops)
  - 1.1.1 [Visual representation of the idea](#visual-representation-of-the-idea)
  - 1.1.2 [Chained Sandwhich](#explaintion-of-the-chained-sandwhich-sandwhich)

2. [title](#title)
3. [title](#title)

## **New type of sandwhich ops**

- **New type of sandwhich ops:** Basically what this "New type" will be or do is that it will take advantage of normal usual sandwiching, chains them together e.g. It would do normal sandwiching <span style="color: green;">**buy**</span>/<span style="color: green;">**Vbuy**</span>/<span style="color: red;">**sell**</span> and then after when it detects a **sell** order from someone it would **sell** again so it would do this: <span style="color: red;">**sell**</span>/<span style="color: red;">**Vsell**</span>/<span style="color: green;">**buy**</span>. So in the end it will look something like this: <span style="color: green;">**buy**</span>/<span style="color: green;">**Vbuy**</span>/<span style="color: red;">**sell**</span>/<span style="color: red;">**sell**</span>/<span style="color: red;">**Vsell**</span>/<span style="color: green;">**buy**</span>.

### **Deeper explaination of "New type of sandwhich ops"**

- #### **Visual representation of the idea:**
  ##### _This is a very basic example_
  <img src="./markdown_images/chain-sandwhich.png"  width="320" height="400">
- #### **Explaintion of the chained-sandwhich method**
  - ## BEFORE CONTINUING TEST IF THE SLIPPAGE WILL AFFECT BOTH THE PRICE UPWARDS AND DOWNWARDS AFTER A BUY AND SELL ORDER BY THE MEV BOT 
  - So again, the basic example shows that we can <span style="color: green;">**buy**</span>/<span style="color: green;">**Vbuy**</span>/<span style="color: red;">**sell**</span>/<span style="color: red;">**sell**</span>/<span style="color: red;">**Vsell**</span>/<span style="color: green;">**buy**</span>. But we can expand this depending on the transactions waiting in the mempool. Let's say we see a lot of buying transactions and a lot of selling transactions for a specific token. In this case, we can then do this:
    <span style="color: green;">**buy**</span>/<span style="color: green;">**Vbuy**</span>/<span style="color: green;">**Vbuy**</span>/<span style="color: green;">**Vbuy**</span>/<span style="color: green;">**Vbuy**</span>/<span style="color: green;">**Vbuy**</span>/<span style="color: red;">**Lsell**</span>/<span style="color: red;">**Vsell**</span>/<span style="color: red;">**Vsell**</span>/<span style="color: red;">**Vsell**</span>/<span style="color: green;">**Lbuy**</span>.

# 📋 Simple Todo App (Rust + Axum), fintrack (Rust + Axum)

A simple backend-only app built with **Rust** and **Axum**.

> ⚠️ **Work In Progress (WIP)**
> ⚠️ **กำลังอยู่ในระหว่างพัฒนา**

---

## 📌 Overview

**EN:**
This is the server-side implementation of a basic Todo and expense tracking app.
Currently, you can interact with it using **cURL** or **Postman** only.
Data is stored temporarily in memory without a database.

**TH:**
แอป Todo ตัวนี้เป็นฝั่ง server-side พัฒนาโดยใช้ **Rust** กับ **Axum**
ตอนนี้ยังใช้งานผ่าน **cURL** หรือ **Postman** ได้เท่านั้น
ข้อมูลที่สร้างจะเก็บอยู่ในหน่วยความจำ (RAM) ชั่วคราว ยังไม่มีระบบฐานข้อมูลถาวร

---

## 📦 Tech Stack

- [Rust](https://www.rust-lang.org/)
- [Axum](https://github.com/tokio-rs/axum)
- [Tokio](https://tokio.rs/)
- [Serde](https://serde.rs/)
- [UUID](https://docs.rs/uuid/)

---

## 🚧 Future Plans / แผนการต่อไป

- 🖥️ Frontend web UI using **Next.js** **for [fintrack](https://github.com/dvloplerz/github_resume/fintrack.git)**
- 🖥️ CLI client using **[Ratatui](https://ratatui.rs/)** **for [sta](https://github.com/dvloplerz/github_resume/simple-todo.git) **
- 🗄️ Add persistent database support (e.g. PostgreSQL, SQLite)  
- 🐳 Dockerize for deployment

---

## 📖 How to Run / วิธีรัน

**EN:**

```bash
cd app_name
cargo run --release

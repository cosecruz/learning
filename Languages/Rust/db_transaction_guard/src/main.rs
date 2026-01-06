struct Connection {
    data: String,
}

#[derive(Debug)]
struct TransactionError {
    state: &'static str,
    message: &'static str,
}

struct Transaction<'conn> {
    conn: &'conn mut Connection,
    committed: bool,
}

impl<'conn> Transaction<'conn> {
    fn begin(conn: &'conn mut Connection) -> Self {
        println!("BEGIN TRANSACTION");
        Self {
            conn,
            committed: false,
        }
    }

    fn execute_tx(&mut self, sql: &str) -> Result<(), TransactionError> {
        if sql.is_empty() {
            return Err(TransactionError {
                state: "in_flight",
                message: "SQL query missing",
            });
        }

        println!("EXECUTE: {}", sql);
        Ok(())
    }

    fn commit(mut self) {
        println!("COMMIT TRANSACTION");
        self.committed = true;
    }

    fn rollback(&mut self) {
        println!("ROLLBACK TRANSACTION");
    }

    // Begin nested transaction
    fn begin_nested(&mut self) -> NestedTransaction<'_, 'conn> {
        NestedTransaction::new(self)
    }
}

impl Drop for Transaction<'_> {
    fn drop(&mut self) {
        if !self.committed {
            self.rollback();
        }
    }
}

struct NestedTransaction<'outer, 'conn> {
    outer: &'outer mut Transaction<'conn>,
    committed: bool,
}

impl<'outer, 'conn> NestedTransaction<'outer, 'conn> {
    fn new(outer: &'outer mut Transaction<'conn>) -> Self {
        println!("SAVEPOINT BEGIN");
        Self {
            outer,
            committed: false,
        }
    }

    fn execute(&mut self, sql: &str) -> Result<(), TransactionError> {
        if sql.is_empty() {
            return Err(TransactionError {
                state: "nested",
                message: "SQL query missing",
            });
        }

        println!("EXECUTE (nested): {}", sql);
        Ok(())
    }

    fn commit(mut self) {
        println!("RELEASE SAVEPOINT");
        self.committed = true;
    }

    fn rollback(&mut self) {
        println!("ROLLBACK TO SAVEPOINT");
    }
}

impl Drop for NestedTransaction<'_, '_> {
    fn drop(&mut self) {
        if !self.committed {
            self.rollback();
        }
    }
}

fn main() -> Result<(), TransactionError> {
    let mut conn = Connection {
        data: "db".to_string(),
    };

    // Outer transaction
    let mut tx = Transaction::begin(&mut conn);
    tx.execute_tx("SELECT * FROM Users")?;

    // Nested transaction
    {
        let mut nested = tx.begin_nested();
        nested.execute("INSERT INTO Addresses VALUES (...)")?;
        // nested.commit(); // Optional: if omitted, Drop rolls back to savepoint
    }

    tx.commit(); // Commit outer transaction
    Ok(())
}

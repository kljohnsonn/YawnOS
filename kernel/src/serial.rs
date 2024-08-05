use core::arch::asm;
//const PORT: u16 = 0x3f8;

#[repr(transparent)]
#[derive(Debug, Clone, Copy)]
pub struct PORT(u16);

impl PORT {
    pub unsafe fn new(port: u16) -> Self {
        PORT(port)
    }

    pub unsafe fn inb(&self, offset: u16) -> u8 {
        let ret: u8;
        asm!("in al, dx", out("al") ret, in("dx") self.0 + offset);
        ret
    }

    pub unsafe fn outb(&self, offset: u16, val: u8) {
        asm!("out dx, al", in("dx") self.0 + offset, in("al") val);
    }

    pub unsafe fn recieved(&self) -> u8 {
        return self.inb(5) & 1;
    }

    pub unsafe fn read(&self) -> u8 {
        while self.recieved() == 0 {}

        return self.inb(self.0);
    }

    pub unsafe fn change(&self, port: u16) -> Self {
        PORT(port)
    }
}

#[derive(Debug)]
pub struct Serial {
    pub port: PORT,
}

impl Serial {
    pub unsafe fn new(port: PORT) -> Option<Self> {
        port.outb(1, 0x00); // Disable all interrupts
        port.outb(3, 0x80); // Enable DLAB (set baud rate divisor)
        port.outb(0, 0x03); // Set divisor to 3 (lo byte) 38400 baud
        port.outb(1, 0x00); //                  (hi byte)
        port.outb(3, 0x03); // 8 bits, no parity, one stop bit
        port.outb(2, 0xC7); // Enable FIFO, clear them, with 14-byte threshold
        port.outb(4, 0x0B); // IRQs enabled, RTS/DSR set
        port.outb(4, 0x1E); // Set in loopback mode, test the serial chip
        port.outb(0, 0xAE); // Test serial chip (send byte 0xAE and check if serial returns same byte)

        if port.inb(0) != 0xAE {
            None
        } else {
            port.outb(4, 0x0F);
            let serial = Serial { port };
            Some(serial)
        }
    }

    pub unsafe fn is_transmit_empty(&self) -> bool {
        if self.port.inb(5) & 0x20 == 0 {
            return true;
        } else {
            return false;
        }
    }

    pub unsafe fn write(&self, string: &str) {
        while self.is_transmit_empty() {}

        let string_bytes = string.as_bytes();

        for s in string_bytes.iter() {
            self.port.outb(0, *s);
        }
    }
}

#[derive(Debug)]
pub enum SerialStatus {
    Success,
}

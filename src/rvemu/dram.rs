use super::*;

#[allow(clippy::upper_case_acronyms)]
pub struct DRAM {
    data: Vec<u8>,
    base: u32,
}

impl DRAM {
    fn align_up(x: u32, align: u32) -> u32 {
        (x + align - 1) / align * align
    }

    pub fn new(data: &[u8], base: u32, size: u32) -> Self {
        let len = DRAM::align_up(size, 4) as usize;
        let mut dram = DRAM {
            data: vec![0; len],
            base,
        };
        dram.data[..data.len()].copy_from_slice(data);
        dram
    }

    pub fn load(&self, addr: u32, size: u32) -> Result<u32> {
        if self.base <= addr && addr < self.base + self.data.len() as u32 {
            let offset = (addr - self.base) as usize;
            match size {
                8 => {
                    let data = self.data[offset] as u32;
                    Ok(data)
                }
                16 => {
                    let data = (self.data[offset] as u32) | ((self.data[offset + 1] as u32) << 8);
                    Ok(data)
                }
                32 => {
                    let data = (self.data[offset] as u32)
                        | ((self.data[offset + 1] as u32) << 8)
                        | ((self.data[offset + 2] as u32) << 16)
                        | ((self.data[offset + 3] as u32) << 24);
                    Ok(data)
                }
                _ => Err(anyhow!("Invalid data size: {}", size)).with_context(|| context!()),
            }
        } else {
            Err(anyhow!("Invalid data address: 0x{:08x}", addr)).with_context(|| context!())
        }
    }

    pub fn store(&mut self, addr: u32, data: u32, size: u32) -> Result<()> {
        if self.base <= addr && addr < self.base + self.data.len() as u32 {
            let offset = (addr - self.base) as usize;
            match size {
                8 => {
                    self.data[offset] = data as u8;
                    Ok(())
                }
                16 => {
                    self.data[offset] = data as u8;
                    self.data[offset + 1] = (data >> 8) as u8;
                    Ok(())
                }
                32 => {
                    self.data[offset] = data as u8;
                    self.data[offset + 1] = (data >> 8) as u8;
                    self.data[offset + 2] = (data >> 16) as u8;
                    self.data[offset + 3] = (data >> 24) as u8;
                    Ok(())
                }
                _ => Err(anyhow!("Invalid data size: {}", size)).with_context(|| context!()),
            }
        } else {
            Err(anyhow!("Invalid data address: 0x{:08x}", addr)).with_context(|| context!())
        }
    }
}
#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::I2SWCLKDIV {
    #[doc = r" Modifies the contents of the register"]
    #[inline]
    pub fn modify<F>(&self, f: F)
    where
        for<'w> F: FnOnce(&R, &'w mut W) -> &'w mut W,
    {
        let bits = self.register.get();
        let r = R { bits: bits };
        let mut w = W { bits: bits };
        f(&r, &mut w);
        self.register.set(w.bits);
    }
    #[doc = r" Reads the contents of the register"]
    #[inline]
    pub fn read(&self) -> R {
        R {
            bits: self.register.get(),
        }
    }
    #[doc = r" Writes to the register"]
    #[inline]
    pub fn write<F>(&self, f: F)
    where
        F: FnOnce(&mut W) -> &mut W,
    {
        let mut w = W::reset_value();
        f(&mut w);
        self.register.set(w.bits);
    }
    #[doc = r" Writes the reset value to the register"]
    #[inline]
    pub fn reset(&self) {
        self.write(|w| w)
    }
}
#[doc = r" Value of the field"]
pub struct RESERVED16R {
    bits: u16,
}
impl RESERVED16R {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u16 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct WDIVR {
    bits: u16,
}
impl WDIVR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u16 {
        self.bits
    }
}
#[doc = r" Proxy"]
pub struct _RESERVED16W<'a> {
    w: &'a mut W,
}
impl<'a> _RESERVED16W<'a> {
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        const MASK: u16 = 65535;
        const OFFSET: u8 = 16;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _WDIVW<'a> {
    w: &'a mut W,
}
impl<'a> _WDIVW<'a> {
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        const MASK: u16 = 65535;
        const OFFSET: u8 = 0;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
impl R {
    #[doc = r" Value of the register as raw bits"]
    #[inline]
    pub fn bits(&self) -> u32 {
        self.bits
    }
    #[doc = "Bits 16:31 - 31:16\\] Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline]
    pub fn reserved16(&self) -> RESERVED16R {
        let bits = {
            const MASK: u16 = 65535;
            const OFFSET: u8 = 16;
            ((self.bits >> OFFSET) & MASK as u32) as u16
        };
        RESERVED16R { bits }
    }
    #[doc = "Bits 0:15 - 15:0\\] If I2SCLKCTL.WCLK_PHASE = 0, Single phase. WCLK is high one BCLK period and low WDIV\\[9:0\\] (unsigned, \\[1-1023\\]) BCLK periods. WCLK = MCUCLK / BDIV*(WDIV\\[9:0\\] + 1) \\[Hz\\] MCUCLK is 48MHz. If I2SCLKCTL.WCLK_PHASE = 1, Dual phase. Each phase on WCLK (50% duty cycle) is WDIV\\[9:0\\] (unsigned, \\[1-1023\\]) BCLK periods. WCLK = MCUCLK / BDIV*(2*WDIV\\[9:0\\]) \\[Hz\\] If I2SCLKCTL.WCLK_PHASE = 2, User defined. WCLK is high WDIV\\[7:0\\] (unsigned, \\[1-255\\]) BCLK periods and low WDIV\\[15:8\\] (unsigned, \\[1-255\\]) BCLK periods. WCLK = MCUCLK / (BDIV*(WDIV\\[7:0\\] + WDIV\\[15:8\\]) \\[Hz\\] For changes to take effect, CLKLOADCTL.LOAD needs to be written"]
    #[inline]
    pub fn wdiv(&self) -> WDIVR {
        let bits = {
            const MASK: u16 = 65535;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) as u16
        };
        WDIVR { bits }
    }
}
impl W {
    #[doc = r" Reset value of the register"]
    #[inline]
    pub fn reset_value() -> W {
        W { bits: 0 }
    }
    #[doc = r" Writes raw bits to the register"]
    #[inline]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
    #[doc = "Bits 16:31 - 31:16\\] Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline]
    pub fn reserved16(&mut self) -> _RESERVED16W {
        _RESERVED16W { w: self }
    }
    #[doc = "Bits 0:15 - 15:0\\] If I2SCLKCTL.WCLK_PHASE = 0, Single phase. WCLK is high one BCLK period and low WDIV\\[9:0\\] (unsigned, \\[1-1023\\]) BCLK periods. WCLK = MCUCLK / BDIV*(WDIV\\[9:0\\] + 1) \\[Hz\\] MCUCLK is 48MHz. If I2SCLKCTL.WCLK_PHASE = 1, Dual phase. Each phase on WCLK (50% duty cycle) is WDIV\\[9:0\\] (unsigned, \\[1-1023\\]) BCLK periods. WCLK = MCUCLK / BDIV*(2*WDIV\\[9:0\\]) \\[Hz\\] If I2SCLKCTL.WCLK_PHASE = 2, User defined. WCLK is high WDIV\\[7:0\\] (unsigned, \\[1-255\\]) BCLK periods and low WDIV\\[15:8\\] (unsigned, \\[1-255\\]) BCLK periods. WCLK = MCUCLK / (BDIV*(WDIV\\[7:0\\] + WDIV\\[15:8\\]) \\[Hz\\] For changes to take effect, CLKLOADCTL.LOAD needs to be written"]
    #[inline]
    pub fn wdiv(&mut self) -> _WDIVW {
        _WDIVW { w: self }
    }
}

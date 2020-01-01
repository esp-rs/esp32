#[doc = "Reader of register RXFIFO_ST"]
pub type R = crate::R<u32, super::RXFIFO_ST>;
#[doc = "Writer for register RXFIFO_ST"]
pub type W = crate::W<u32, super::RXFIFO_ST>;
#[doc = "Register RXFIFO_ST `reset()`'s with value 0"]
impl crate::ResetValue for super::RXFIFO_ST {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `I2C_TXFIFO_END_ADDR`"]
pub type I2C_TXFIFO_END_ADDR_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `I2C_TXFIFO_END_ADDR`"]
pub struct I2C_TXFIFO_END_ADDR_W<'a> {
    w: &'a mut W,
}
impl<'a> I2C_TXFIFO_END_ADDR_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x1f << 15)) | (((value as u32) & 0x1f) << 15);
        self.w
    }
}
#[doc = "Reader of field `I2C_TXFIFO_START_ADDR`"]
pub type I2C_TXFIFO_START_ADDR_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `I2C_TXFIFO_START_ADDR`"]
pub struct I2C_TXFIFO_START_ADDR_W<'a> {
    w: &'a mut W,
}
impl<'a> I2C_TXFIFO_START_ADDR_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x1f << 10)) | (((value as u32) & 0x1f) << 10);
        self.w
    }
}
#[doc = "Reader of field `I2C_RXFIFO_END_ADDR`"]
pub type I2C_RXFIFO_END_ADDR_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `I2C_RXFIFO_END_ADDR`"]
pub struct I2C_RXFIFO_END_ADDR_W<'a> {
    w: &'a mut W,
}
impl<'a> I2C_RXFIFO_END_ADDR_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x1f << 5)) | (((value as u32) & 0x1f) << 5);
        self.w
    }
}
#[doc = "Reader of field `I2C_RXFIFO_START_ADDR`"]
pub type I2C_RXFIFO_START_ADDR_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `I2C_RXFIFO_START_ADDR`"]
pub struct I2C_RXFIFO_START_ADDR_W<'a> {
    w: &'a mut W,
}
impl<'a> I2C_RXFIFO_START_ADDR_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x1f) | ((value as u32) & 0x1f);
        self.w
    }
}
impl R {
    #[doc = "Bits 15:19 - This is the offset address of the last sending data as described in nonfifo_tx_thres register."]
    #[inline(always)]
    pub fn i2c_txfifo_end_addr(&self) -> I2C_TXFIFO_END_ADDR_R {
        I2C_TXFIFO_END_ADDR_R::new(((self.bits >> 15) & 0x1f) as u8)
    }
    #[doc = "Bits 10:14 - This is the offset address of the first sending data as described in nonfifo_tx_thres register."]
    #[inline(always)]
    pub fn i2c_txfifo_start_addr(&self) -> I2C_TXFIFO_START_ADDR_R {
        I2C_TXFIFO_START_ADDR_R::new(((self.bits >> 10) & 0x1f) as u8)
    }
    #[doc = "Bits 5:9 - This is the offset address of the first receiving data as described in nonfifo_rx_thres_register."]
    #[inline(always)]
    pub fn i2c_rxfifo_end_addr(&self) -> I2C_RXFIFO_END_ADDR_R {
        I2C_RXFIFO_END_ADDR_R::new(((self.bits >> 5) & 0x1f) as u8)
    }
    #[doc = "Bits 0:4 - This is the offset address of the last receiving data as described in nonfifo_rx_thres_register."]
    #[inline(always)]
    pub fn i2c_rxfifo_start_addr(&self) -> I2C_RXFIFO_START_ADDR_R {
        I2C_RXFIFO_START_ADDR_R::new((self.bits & 0x1f) as u8)
    }
}
impl W {
    #[doc = "Bits 15:19 - This is the offset address of the last sending data as described in nonfifo_tx_thres register."]
    #[inline(always)]
    pub fn i2c_txfifo_end_addr(&mut self) -> I2C_TXFIFO_END_ADDR_W {
        I2C_TXFIFO_END_ADDR_W { w: self }
    }
    #[doc = "Bits 10:14 - This is the offset address of the first sending data as described in nonfifo_tx_thres register."]
    #[inline(always)]
    pub fn i2c_txfifo_start_addr(&mut self) -> I2C_TXFIFO_START_ADDR_W {
        I2C_TXFIFO_START_ADDR_W { w: self }
    }
    #[doc = "Bits 5:9 - This is the offset address of the first receiving data as described in nonfifo_rx_thres_register."]
    #[inline(always)]
    pub fn i2c_rxfifo_end_addr(&mut self) -> I2C_RXFIFO_END_ADDR_W {
        I2C_RXFIFO_END_ADDR_W { w: self }
    }
    #[doc = "Bits 0:4 - This is the offset address of the last receiving data as described in nonfifo_rx_thres_register."]
    #[inline(always)]
    pub fn i2c_rxfifo_start_addr(&mut self) -> I2C_RXFIFO_START_ADDR_W {
        I2C_RXFIFO_START_ADDR_W { w: self }
    }
}

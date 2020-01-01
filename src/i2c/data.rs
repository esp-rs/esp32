#[doc = "Reader of register DATA"]
pub type R = crate::R<u32, super::DATA>;
#[doc = "Writer for register DATA"]
pub type W = crate::W<u32, super::DATA>;
#[doc = "Register DATA `reset()`'s with value 0"]
impl crate::ResetValue for super::DATA {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `I2C_FIFO_RDATA`"]
pub type I2C_FIFO_RDATA_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `I2C_FIFO_RDATA`"]
pub struct I2C_FIFO_RDATA_W<'a> {
    w: &'a mut W,
}
impl<'a> I2C_FIFO_RDATA_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xff) | ((value as u32) & 0xff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:7 - The register represent the byte data read from rxfifo when use apb fifo access"]
    #[inline(always)]
    pub fn i2c_fifo_rdata(&self) -> I2C_FIFO_RDATA_R {
        I2C_FIFO_RDATA_R::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - The register represent the byte data read from rxfifo when use apb fifo access"]
    #[inline(always)]
    pub fn i2c_fifo_rdata(&mut self) -> I2C_FIFO_RDATA_W {
        I2C_FIFO_RDATA_W { w: self }
    }
}

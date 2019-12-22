#[doc = "Reader of register I2C_SLAVE_ADDR_REG"]
pub type R = crate::R<u32, super::I2C_SLAVE_ADDR_REG>;
#[doc = "Writer for register I2C_SLAVE_ADDR_REG"]
pub type W = crate::W<u32, super::I2C_SLAVE_ADDR_REG>;
#[doc = "Register I2C_SLAVE_ADDR_REG `reset()`'s with value 0"]
impl crate::ResetValue for super::I2C_SLAVE_ADDR_REG {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `I2C_ADDR_10BIT_EN`"]
pub type I2C_ADDR_10BIT_EN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `I2C_ADDR_10BIT_EN`"]
pub struct I2C_ADDR_10BIT_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> I2C_ADDR_10BIT_EN_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 31)) | (((value as u32) & 0x01) << 31);
        self.w
    }
}
#[doc = "Reader of field `I2C_SLAVE_ADDR`"]
pub type I2C_SLAVE_ADDR_R = crate::R<u16, u16>;
#[doc = "Write proxy for field `I2C_SLAVE_ADDR`"]
pub struct I2C_SLAVE_ADDR_W<'a> {
    w: &'a mut W,
}
impl<'a> I2C_SLAVE_ADDR_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x7fff) | ((value as u32) & 0x7fff);
        self.w
    }
}
impl R {
    #[doc = "Bit 31 - This register is used to enable slave 10bit address mode."]
    #[inline(always)]
    pub fn i2c_addr_10bit_en(&self) -> I2C_ADDR_10BIT_EN_R {
        I2C_ADDR_10BIT_EN_R::new(((self.bits >> 31) & 0x01) != 0)
    }
    #[doc = "Bits 0:14 - when configured as i2c slave this register is used to configure slave's address."]
    #[inline(always)]
    pub fn i2c_slave_addr(&self) -> I2C_SLAVE_ADDR_R {
        I2C_SLAVE_ADDR_R::new((self.bits & 0x7fff) as u16)
    }
}
impl W {
    #[doc = "Bit 31 - This register is used to enable slave 10bit address mode."]
    #[inline(always)]
    pub fn i2c_addr_10bit_en(&mut self) -> I2C_ADDR_10BIT_EN_W {
        I2C_ADDR_10BIT_EN_W { w: self }
    }
    #[doc = "Bits 0:14 - when configured as i2c slave this register is used to configure slave's address."]
    #[inline(always)]
    pub fn i2c_slave_addr(&mut self) -> I2C_SLAVE_ADDR_W {
        I2C_SLAVE_ADDR_W { w: self }
    }
}

#[doc = "Reader of register SAR_SLAVE_ADDR3"]
pub type R = crate::R<u32, super::SAR_SLAVE_ADDR3>;
#[doc = "Writer for register SAR_SLAVE_ADDR3"]
pub type W = crate::W<u32, super::SAR_SLAVE_ADDR3>;
#[doc = "Register SAR_SLAVE_ADDR3 `reset()`'s with value 0"]
impl crate::ResetValue for super::SAR_SLAVE_ADDR3 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `TSENS_RDY_OUT`"]
pub type TSENS_RDY_OUT_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `TSENS_RDY_OUT`"]
pub struct TSENS_RDY_OUT_W<'a> {
    w: &'a mut W,
}
impl<'a> TSENS_RDY_OUT_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 30)) | (((value as u32) & 0x01) << 30);
        self.w
    }
}
#[doc = "Reader of field `TSENS_OUT`"]
pub type TSENS_OUT_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `TSENS_OUT`"]
pub struct TSENS_OUT_W<'a> {
    w: &'a mut W,
}
impl<'a> TSENS_OUT_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xff << 22)) | (((value as u32) & 0xff) << 22);
        self.w
    }
}
#[doc = "Reader of field `I2C_SLAVE_ADDR4`"]
pub type I2C_SLAVE_ADDR4_R = crate::R<u16, u16>;
#[doc = "Write proxy for field `I2C_SLAVE_ADDR4`"]
pub struct I2C_SLAVE_ADDR4_W<'a> {
    w: &'a mut W,
}
impl<'a> I2C_SLAVE_ADDR4_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07ff << 11)) | (((value as u32) & 0x07ff) << 11);
        self.w
    }
}
#[doc = "Reader of field `I2C_SLAVE_ADDR5`"]
pub type I2C_SLAVE_ADDR5_R = crate::R<u16, u16>;
#[doc = "Write proxy for field `I2C_SLAVE_ADDR5`"]
pub struct I2C_SLAVE_ADDR5_W<'a> {
    w: &'a mut W,
}
impl<'a> I2C_SLAVE_ADDR5_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x07ff) | ((value as u32) & 0x07ff);
        self.w
    }
}
impl R {
    #[doc = "Bit 30 - indicate temperature sensor out ready"]
    #[inline(always)]
    pub fn tsens_rdy_out(&self) -> TSENS_RDY_OUT_R {
        TSENS_RDY_OUT_R::new(((self.bits >> 30) & 0x01) != 0)
    }
    #[doc = "Bits 22:29 - temperature sensor data out"]
    #[inline(always)]
    pub fn tsens_out(&self) -> TSENS_OUT_R {
        TSENS_OUT_R::new(((self.bits >> 22) & 0xff) as u8)
    }
    #[doc = "Bits 11:21"]
    #[inline(always)]
    pub fn i2c_slave_addr4(&self) -> I2C_SLAVE_ADDR4_R {
        I2C_SLAVE_ADDR4_R::new(((self.bits >> 11) & 0x07ff) as u16)
    }
    #[doc = "Bits 0:10"]
    #[inline(always)]
    pub fn i2c_slave_addr5(&self) -> I2C_SLAVE_ADDR5_R {
        I2C_SLAVE_ADDR5_R::new((self.bits & 0x07ff) as u16)
    }
}
impl W {
    #[doc = "Bit 30 - indicate temperature sensor out ready"]
    #[inline(always)]
    pub fn tsens_rdy_out(&mut self) -> TSENS_RDY_OUT_W {
        TSENS_RDY_OUT_W { w: self }
    }
    #[doc = "Bits 22:29 - temperature sensor data out"]
    #[inline(always)]
    pub fn tsens_out(&mut self) -> TSENS_OUT_W {
        TSENS_OUT_W { w: self }
    }
    #[doc = "Bits 11:21"]
    #[inline(always)]
    pub fn i2c_slave_addr4(&mut self) -> I2C_SLAVE_ADDR4_W {
        I2C_SLAVE_ADDR4_W { w: self }
    }
    #[doc = "Bits 0:10"]
    #[inline(always)]
    pub fn i2c_slave_addr5(&mut self) -> I2C_SLAVE_ADDR5_W {
        I2C_SLAVE_ADDR5_W { w: self }
    }
}

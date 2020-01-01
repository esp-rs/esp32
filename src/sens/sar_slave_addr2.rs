#[doc = "Reader of register SAR_SLAVE_ADDR2"]
pub type R = crate::R<u32, super::SAR_SLAVE_ADDR2>;
#[doc = "Writer for register SAR_SLAVE_ADDR2"]
pub type W = crate::W<u32, super::SAR_SLAVE_ADDR2>;
#[doc = "Register SAR_SLAVE_ADDR2 `reset()`'s with value 0"]
impl crate::ResetValue for super::SAR_SLAVE_ADDR2 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `SENS_I2C_SLAVE_ADDR2`"]
pub type SENS_I2C_SLAVE_ADDR2_R = crate::R<u16, u16>;
#[doc = "Write proxy for field `SENS_I2C_SLAVE_ADDR2`"]
pub struct SENS_I2C_SLAVE_ADDR2_W<'a> {
    w: &'a mut W,
}
impl<'a> SENS_I2C_SLAVE_ADDR2_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07ff << 11)) | (((value as u32) & 0x07ff) << 11);
        self.w
    }
}
#[doc = "Reader of field `SENS_I2C_SLAVE_ADDR3`"]
pub type SENS_I2C_SLAVE_ADDR3_R = crate::R<u16, u16>;
#[doc = "Write proxy for field `SENS_I2C_SLAVE_ADDR3`"]
pub struct SENS_I2C_SLAVE_ADDR3_W<'a> {
    w: &'a mut W,
}
impl<'a> SENS_I2C_SLAVE_ADDR3_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x07ff) | ((value as u32) & 0x07ff);
        self.w
    }
}
impl R {
    #[doc = "Bits 11:21"]
    #[inline(always)]
    pub fn sens_i2c_slave_addr2(&self) -> SENS_I2C_SLAVE_ADDR2_R {
        SENS_I2C_SLAVE_ADDR2_R::new(((self.bits >> 11) & 0x07ff) as u16)
    }
    #[doc = "Bits 0:10"]
    #[inline(always)]
    pub fn sens_i2c_slave_addr3(&self) -> SENS_I2C_SLAVE_ADDR3_R {
        SENS_I2C_SLAVE_ADDR3_R::new((self.bits & 0x07ff) as u16)
    }
}
impl W {
    #[doc = "Bits 11:21"]
    #[inline(always)]
    pub fn sens_i2c_slave_addr2(&mut self) -> SENS_I2C_SLAVE_ADDR2_W {
        SENS_I2C_SLAVE_ADDR2_W { w: self }
    }
    #[doc = "Bits 0:10"]
    #[inline(always)]
    pub fn sens_i2c_slave_addr3(&mut self) -> SENS_I2C_SLAVE_ADDR3_W {
        SENS_I2C_SLAVE_ADDR3_W { w: self }
    }
}

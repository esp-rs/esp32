#[doc = "Reader of register SENS_SAR_SLAVE_ADDR1_REG"]
pub type R = crate::R<u32, super::SENS_SAR_SLAVE_ADDR1_REG>;
#[doc = "Writer for register SENS_SAR_SLAVE_ADDR1_REG"]
pub type W = crate::W<u32, super::SENS_SAR_SLAVE_ADDR1_REG>;
#[doc = "Register SENS_SAR_SLAVE_ADDR1_REG `reset()`'s with value 0"]
impl crate::ResetValue for super::SENS_SAR_SLAVE_ADDR1_REG {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `SENS_MEAS_STATUS`"]
pub type SENS_MEAS_STATUS_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `SENS_MEAS_STATUS`"]
pub struct SENS_MEAS_STATUS_W<'a> {
    w: &'a mut W,
}
impl<'a> SENS_MEAS_STATUS_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xff << 22)) | (((value as u32) & 0xff) << 22);
        self.w
    }
}
#[doc = "Reader of field `SENS_I2C_SLAVE_ADDR0`"]
pub type SENS_I2C_SLAVE_ADDR0_R = crate::R<u16, u16>;
#[doc = "Write proxy for field `SENS_I2C_SLAVE_ADDR0`"]
pub struct SENS_I2C_SLAVE_ADDR0_W<'a> {
    w: &'a mut W,
}
impl<'a> SENS_I2C_SLAVE_ADDR0_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07ff << 11)) | (((value as u32) & 0x07ff) << 11);
        self.w
    }
}
#[doc = "Reader of field `SENS_I2C_SLAVE_ADDR1`"]
pub type SENS_I2C_SLAVE_ADDR1_R = crate::R<u16, u16>;
#[doc = "Write proxy for field `SENS_I2C_SLAVE_ADDR1`"]
pub struct SENS_I2C_SLAVE_ADDR1_W<'a> {
    w: &'a mut W,
}
impl<'a> SENS_I2C_SLAVE_ADDR1_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x07ff) | ((value as u32) & 0x07ff);
        self.w
    }
}
impl R {
    #[doc = "Bits 22:29"]
    #[inline(always)]
    pub fn sens_meas_status(&self) -> SENS_MEAS_STATUS_R {
        SENS_MEAS_STATUS_R::new(((self.bits >> 22) & 0xff) as u8)
    }
    #[doc = "Bits 11:21"]
    #[inline(always)]
    pub fn sens_i2c_slave_addr0(&self) -> SENS_I2C_SLAVE_ADDR0_R {
        SENS_I2C_SLAVE_ADDR0_R::new(((self.bits >> 11) & 0x07ff) as u16)
    }
    #[doc = "Bits 0:10"]
    #[inline(always)]
    pub fn sens_i2c_slave_addr1(&self) -> SENS_I2C_SLAVE_ADDR1_R {
        SENS_I2C_SLAVE_ADDR1_R::new((self.bits & 0x07ff) as u16)
    }
}
impl W {
    #[doc = "Bits 22:29"]
    #[inline(always)]
    pub fn sens_meas_status(&mut self) -> SENS_MEAS_STATUS_W {
        SENS_MEAS_STATUS_W { w: self }
    }
    #[doc = "Bits 11:21"]
    #[inline(always)]
    pub fn sens_i2c_slave_addr0(&mut self) -> SENS_I2C_SLAVE_ADDR0_W {
        SENS_I2C_SLAVE_ADDR0_W { w: self }
    }
    #[doc = "Bits 0:10"]
    #[inline(always)]
    pub fn sens_i2c_slave_addr1(&mut self) -> SENS_I2C_SLAVE_ADDR1_W {
        SENS_I2C_SLAVE_ADDR1_W { w: self }
    }
}

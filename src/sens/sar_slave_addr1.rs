#[doc = "Reader of register SAR_SLAVE_ADDR1"]
pub type R = crate::R<u32, super::SAR_SLAVE_ADDR1>;
#[doc = "Writer for register SAR_SLAVE_ADDR1"]
pub type W = crate::W<u32, super::SAR_SLAVE_ADDR1>;
#[doc = "Register SAR_SLAVE_ADDR1 `reset()`'s with value 0"]
impl crate::ResetValue for super::SAR_SLAVE_ADDR1 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `MEAS_STATUS`"]
pub type MEAS_STATUS_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `MEAS_STATUS`"]
pub struct MEAS_STATUS_W<'a> {
    w: &'a mut W,
}
impl<'a> MEAS_STATUS_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xff << 22)) | (((value as u32) & 0xff) << 22);
        self.w
    }
}
#[doc = "Reader of field `I2C_SLAVE_ADDR0`"]
pub type I2C_SLAVE_ADDR0_R = crate::R<u16, u16>;
#[doc = "Write proxy for field `I2C_SLAVE_ADDR0`"]
pub struct I2C_SLAVE_ADDR0_W<'a> {
    w: &'a mut W,
}
impl<'a> I2C_SLAVE_ADDR0_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07ff << 11)) | (((value as u32) & 0x07ff) << 11);
        self.w
    }
}
#[doc = "Reader of field `I2C_SLAVE_ADDR1`"]
pub type I2C_SLAVE_ADDR1_R = crate::R<u16, u16>;
#[doc = "Write proxy for field `I2C_SLAVE_ADDR1`"]
pub struct I2C_SLAVE_ADDR1_W<'a> {
    w: &'a mut W,
}
impl<'a> I2C_SLAVE_ADDR1_W<'a> {
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
    pub fn meas_status(&self) -> MEAS_STATUS_R {
        MEAS_STATUS_R::new(((self.bits >> 22) & 0xff) as u8)
    }
    #[doc = "Bits 11:21"]
    #[inline(always)]
    pub fn i2c_slave_addr0(&self) -> I2C_SLAVE_ADDR0_R {
        I2C_SLAVE_ADDR0_R::new(((self.bits >> 11) & 0x07ff) as u16)
    }
    #[doc = "Bits 0:10"]
    #[inline(always)]
    pub fn i2c_slave_addr1(&self) -> I2C_SLAVE_ADDR1_R {
        I2C_SLAVE_ADDR1_R::new((self.bits & 0x07ff) as u16)
    }
}
impl W {
    #[doc = "Bits 22:29"]
    #[inline(always)]
    pub fn meas_status(&mut self) -> MEAS_STATUS_W {
        MEAS_STATUS_W { w: self }
    }
    #[doc = "Bits 11:21"]
    #[inline(always)]
    pub fn i2c_slave_addr0(&mut self) -> I2C_SLAVE_ADDR0_W {
        I2C_SLAVE_ADDR0_W { w: self }
    }
    #[doc = "Bits 0:10"]
    #[inline(always)]
    pub fn i2c_slave_addr1(&mut self) -> I2C_SLAVE_ADDR1_W {
        I2C_SLAVE_ADDR1_W { w: self }
    }
}

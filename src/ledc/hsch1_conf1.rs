#[doc = "Reader of register HSCH1_CONF1"]
pub type R = crate::R<u32, super::HSCH1_CONF1>;
#[doc = "Writer for register HSCH1_CONF1"]
pub type W = crate::W<u32, super::HSCH1_CONF1>;
#[doc = "Register HSCH1_CONF1 `reset()`'s with value 0"]
impl crate::ResetValue for super::HSCH1_CONF1 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `DUTY_START_HSCH1`"]
pub type DUTY_START_HSCH1_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `DUTY_START_HSCH1`"]
pub struct DUTY_START_HSCH1_W<'a> {
    w: &'a mut W,
}
impl<'a> DUTY_START_HSCH1_W<'a> {
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
#[doc = "Reader of field `DUTY_INC_HSCH1`"]
pub type DUTY_INC_HSCH1_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `DUTY_INC_HSCH1`"]
pub struct DUTY_INC_HSCH1_W<'a> {
    w: &'a mut W,
}
impl<'a> DUTY_INC_HSCH1_W<'a> {
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
#[doc = "Reader of field `DUTY_NUM_HSCH1`"]
pub type DUTY_NUM_HSCH1_R = crate::R<u16, u16>;
#[doc = "Write proxy for field `DUTY_NUM_HSCH1`"]
pub struct DUTY_NUM_HSCH1_W<'a> {
    w: &'a mut W,
}
impl<'a> DUTY_NUM_HSCH1_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03ff << 20)) | (((value as u32) & 0x03ff) << 20);
        self.w
    }
}
#[doc = "Reader of field `DUTY_CYCLE_HSCH1`"]
pub type DUTY_CYCLE_HSCH1_R = crate::R<u16, u16>;
#[doc = "Write proxy for field `DUTY_CYCLE_HSCH1`"]
pub struct DUTY_CYCLE_HSCH1_W<'a> {
    w: &'a mut W,
}
impl<'a> DUTY_CYCLE_HSCH1_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03ff << 10)) | (((value as u32) & 0x03ff) << 10);
        self.w
    }
}
#[doc = "Reader of field `DUTY_SCALE_HSCH1`"]
pub type DUTY_SCALE_HSCH1_R = crate::R<u16, u16>;
#[doc = "Write proxy for field `DUTY_SCALE_HSCH1`"]
pub struct DUTY_SCALE_HSCH1_W<'a> {
    w: &'a mut W,
}
impl<'a> DUTY_SCALE_HSCH1_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x03ff) | ((value as u32) & 0x03ff);
        self.w
    }
}
impl R {
    #[doc = "Bit 31 - When reg_duty_num_hsch1 reg_duty_cycle_hsch1 and reg_duty_scale_hsch1 has been configured. these register won't take effect until set reg_duty_start_hsch1. this bit is automatically cleared by hardware."]
    #[inline(always)]
    pub fn duty_start_hsch1(&self) -> DUTY_START_HSCH1_R {
        DUTY_START_HSCH1_R::new(((self.bits >> 31) & 0x01) != 0)
    }
    #[doc = "Bit 30 - This register is used to increase the duty of output signal or decrease the duty of output signal for high speed channel1."]
    #[inline(always)]
    pub fn duty_inc_hsch1(&self) -> DUTY_INC_HSCH1_R {
        DUTY_INC_HSCH1_R::new(((self.bits >> 30) & 0x01) != 0)
    }
    #[doc = "Bits 20:29 - This register is used to control the num of increased or decreased times for high speed channel1."]
    #[inline(always)]
    pub fn duty_num_hsch1(&self) -> DUTY_NUM_HSCH1_R {
        DUTY_NUM_HSCH1_R::new(((self.bits >> 20) & 0x03ff) as u16)
    }
    #[doc = "Bits 10:19 - This register is used to increase or decrease the duty every reg_duty_cycle_hsch1 cycles for high speed channel1."]
    #[inline(always)]
    pub fn duty_cycle_hsch1(&self) -> DUTY_CYCLE_HSCH1_R {
        DUTY_CYCLE_HSCH1_R::new(((self.bits >> 10) & 0x03ff) as u16)
    }
    #[doc = "Bits 0:9 - This register controls the increase or decrease step scale for high speed channel1."]
    #[inline(always)]
    pub fn duty_scale_hsch1(&self) -> DUTY_SCALE_HSCH1_R {
        DUTY_SCALE_HSCH1_R::new((self.bits & 0x03ff) as u16)
    }
}
impl W {
    #[doc = "Bit 31 - When reg_duty_num_hsch1 reg_duty_cycle_hsch1 and reg_duty_scale_hsch1 has been configured. these register won't take effect until set reg_duty_start_hsch1. this bit is automatically cleared by hardware."]
    #[inline(always)]
    pub fn duty_start_hsch1(&mut self) -> DUTY_START_HSCH1_W {
        DUTY_START_HSCH1_W { w: self }
    }
    #[doc = "Bit 30 - This register is used to increase the duty of output signal or decrease the duty of output signal for high speed channel1."]
    #[inline(always)]
    pub fn duty_inc_hsch1(&mut self) -> DUTY_INC_HSCH1_W {
        DUTY_INC_HSCH1_W { w: self }
    }
    #[doc = "Bits 20:29 - This register is used to control the num of increased or decreased times for high speed channel1."]
    #[inline(always)]
    pub fn duty_num_hsch1(&mut self) -> DUTY_NUM_HSCH1_W {
        DUTY_NUM_HSCH1_W { w: self }
    }
    #[doc = "Bits 10:19 - This register is used to increase or decrease the duty every reg_duty_cycle_hsch1 cycles for high speed channel1."]
    #[inline(always)]
    pub fn duty_cycle_hsch1(&mut self) -> DUTY_CYCLE_HSCH1_W {
        DUTY_CYCLE_HSCH1_W { w: self }
    }
    #[doc = "Bits 0:9 - This register controls the increase or decrease step scale for high speed channel1."]
    #[inline(always)]
    pub fn duty_scale_hsch1(&mut self) -> DUTY_SCALE_HSCH1_W {
        DUTY_SCALE_HSCH1_W { w: self }
    }
}
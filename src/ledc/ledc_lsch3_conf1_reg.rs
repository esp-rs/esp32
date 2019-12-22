#[doc = "Reader of register LEDC_LSCH3_CONF1_REG"]
pub type R = crate::R<u32, super::LEDC_LSCH3_CONF1_REG>;
#[doc = "Writer for register LEDC_LSCH3_CONF1_REG"]
pub type W = crate::W<u32, super::LEDC_LSCH3_CONF1_REG>;
#[doc = "Register LEDC_LSCH3_CONF1_REG `reset()`'s with value 0"]
impl crate::ResetValue for super::LEDC_LSCH3_CONF1_REG {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `LEDC_DUTY_START_LSCH3`"]
pub type LEDC_DUTY_START_LSCH3_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `LEDC_DUTY_START_LSCH3`"]
pub struct LEDC_DUTY_START_LSCH3_W<'a> {
    w: &'a mut W,
}
impl<'a> LEDC_DUTY_START_LSCH3_W<'a> {
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
#[doc = "Reader of field `LEDC_DUTY_INC_LSCH3`"]
pub type LEDC_DUTY_INC_LSCH3_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `LEDC_DUTY_INC_LSCH3`"]
pub struct LEDC_DUTY_INC_LSCH3_W<'a> {
    w: &'a mut W,
}
impl<'a> LEDC_DUTY_INC_LSCH3_W<'a> {
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
#[doc = "Reader of field `LEDC_DUTY_NUM_LSCH3`"]
pub type LEDC_DUTY_NUM_LSCH3_R = crate::R<u16, u16>;
#[doc = "Write proxy for field `LEDC_DUTY_NUM_LSCH3`"]
pub struct LEDC_DUTY_NUM_LSCH3_W<'a> {
    w: &'a mut W,
}
impl<'a> LEDC_DUTY_NUM_LSCH3_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03ff << 20)) | (((value as u32) & 0x03ff) << 20);
        self.w
    }
}
#[doc = "Reader of field `LEDC_DUTY_CYCLE_LSCH3`"]
pub type LEDC_DUTY_CYCLE_LSCH3_R = crate::R<u16, u16>;
#[doc = "Write proxy for field `LEDC_DUTY_CYCLE_LSCH3`"]
pub struct LEDC_DUTY_CYCLE_LSCH3_W<'a> {
    w: &'a mut W,
}
impl<'a> LEDC_DUTY_CYCLE_LSCH3_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03ff << 10)) | (((value as u32) & 0x03ff) << 10);
        self.w
    }
}
#[doc = "Reader of field `LEDC_DUTY_SCALE_LSCH3`"]
pub type LEDC_DUTY_SCALE_LSCH3_R = crate::R<u16, u16>;
#[doc = "Write proxy for field `LEDC_DUTY_SCALE_LSCH3`"]
pub struct LEDC_DUTY_SCALE_LSCH3_W<'a> {
    w: &'a mut W,
}
impl<'a> LEDC_DUTY_SCALE_LSCH3_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x03ff) | ((value as u32) & 0x03ff);
        self.w
    }
}
impl R {
    #[doc = "Bit 31 - When reg_duty_num_hsch3 reg_duty_cycle_hsch3 and reg_duty_scale_hsch3 has been configured. these register won't take effect until set reg_duty_start_hsch3. this bit is automatically cleared by hardware."]
    #[inline(always)]
    pub fn ledc_duty_start_lsch3(&self) -> LEDC_DUTY_START_LSCH3_R {
        LEDC_DUTY_START_LSCH3_R::new(((self.bits >> 31) & 0x01) != 0)
    }
    #[doc = "Bit 30 - This register is used to increase the duty of output signal or decrease the duty of output signal for low speed channel3."]
    #[inline(always)]
    pub fn ledc_duty_inc_lsch3(&self) -> LEDC_DUTY_INC_LSCH3_R {
        LEDC_DUTY_INC_LSCH3_R::new(((self.bits >> 30) & 0x01) != 0)
    }
    #[doc = "Bits 20:29 - This register is used to control the num of increased or decreased times for low speed channel3."]
    #[inline(always)]
    pub fn ledc_duty_num_lsch3(&self) -> LEDC_DUTY_NUM_LSCH3_R {
        LEDC_DUTY_NUM_LSCH3_R::new(((self.bits >> 20) & 0x03ff) as u16)
    }
    #[doc = "Bits 10:19 - This register is used to increase or decrease the duty every reg_duty_cycle_lsch3 cycles for low speed channel3."]
    #[inline(always)]
    pub fn ledc_duty_cycle_lsch3(&self) -> LEDC_DUTY_CYCLE_LSCH3_R {
        LEDC_DUTY_CYCLE_LSCH3_R::new(((self.bits >> 10) & 0x03ff) as u16)
    }
    #[doc = "Bits 0:9 - This register controls the increase or decrease step scale for low speed channel3."]
    #[inline(always)]
    pub fn ledc_duty_scale_lsch3(&self) -> LEDC_DUTY_SCALE_LSCH3_R {
        LEDC_DUTY_SCALE_LSCH3_R::new((self.bits & 0x03ff) as u16)
    }
}
impl W {
    #[doc = "Bit 31 - When reg_duty_num_hsch3 reg_duty_cycle_hsch3 and reg_duty_scale_hsch3 has been configured. these register won't take effect until set reg_duty_start_hsch3. this bit is automatically cleared by hardware."]
    #[inline(always)]
    pub fn ledc_duty_start_lsch3(&mut self) -> LEDC_DUTY_START_LSCH3_W {
        LEDC_DUTY_START_LSCH3_W { w: self }
    }
    #[doc = "Bit 30 - This register is used to increase the duty of output signal or decrease the duty of output signal for low speed channel3."]
    #[inline(always)]
    pub fn ledc_duty_inc_lsch3(&mut self) -> LEDC_DUTY_INC_LSCH3_W {
        LEDC_DUTY_INC_LSCH3_W { w: self }
    }
    #[doc = "Bits 20:29 - This register is used to control the num of increased or decreased times for low speed channel3."]
    #[inline(always)]
    pub fn ledc_duty_num_lsch3(&mut self) -> LEDC_DUTY_NUM_LSCH3_W {
        LEDC_DUTY_NUM_LSCH3_W { w: self }
    }
    #[doc = "Bits 10:19 - This register is used to increase or decrease the duty every reg_duty_cycle_lsch3 cycles for low speed channel3."]
    #[inline(always)]
    pub fn ledc_duty_cycle_lsch3(&mut self) -> LEDC_DUTY_CYCLE_LSCH3_W {
        LEDC_DUTY_CYCLE_LSCH3_W { w: self }
    }
    #[doc = "Bits 0:9 - This register controls the increase or decrease step scale for low speed channel3."]
    #[inline(always)]
    pub fn ledc_duty_scale_lsch3(&mut self) -> LEDC_DUTY_SCALE_LSCH3_W {
        LEDC_DUTY_SCALE_LSCH3_W { w: self }
    }
}

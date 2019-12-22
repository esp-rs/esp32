#[doc = "Reader of register LEDC_INT_ST_REG"]
pub type R = crate::R<u32, super::LEDC_INT_ST_REG>;
#[doc = "Writer for register LEDC_INT_ST_REG"]
pub type W = crate::W<u32, super::LEDC_INT_ST_REG>;
#[doc = "Register LEDC_INT_ST_REG `reset()`'s with value 0"]
impl crate::ResetValue for super::LEDC_INT_ST_REG {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `LEDC_DUTY_CHNG_END_LSCH7_INT_ST`"]
pub type LEDC_DUTY_CHNG_END_LSCH7_INT_ST_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `LEDC_DUTY_CHNG_END_LSCH7_INT_ST`"]
pub struct LEDC_DUTY_CHNG_END_LSCH7_INT_ST_W<'a> {
    w: &'a mut W,
}
impl<'a> LEDC_DUTY_CHNG_END_LSCH7_INT_ST_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 23)) | (((value as u32) & 0x01) << 23);
        self.w
    }
}
#[doc = "Reader of field `LEDC_DUTY_CHNG_END_LSCH6_INT_ST`"]
pub type LEDC_DUTY_CHNG_END_LSCH6_INT_ST_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `LEDC_DUTY_CHNG_END_LSCH6_INT_ST`"]
pub struct LEDC_DUTY_CHNG_END_LSCH6_INT_ST_W<'a> {
    w: &'a mut W,
}
impl<'a> LEDC_DUTY_CHNG_END_LSCH6_INT_ST_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 22)) | (((value as u32) & 0x01) << 22);
        self.w
    }
}
#[doc = "Reader of field `LEDC_DUTY_CHNG_END_LSCH5_INT_ST`"]
pub type LEDC_DUTY_CHNG_END_LSCH5_INT_ST_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `LEDC_DUTY_CHNG_END_LSCH5_INT_ST`"]
pub struct LEDC_DUTY_CHNG_END_LSCH5_INT_ST_W<'a> {
    w: &'a mut W,
}
impl<'a> LEDC_DUTY_CHNG_END_LSCH5_INT_ST_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 21)) | (((value as u32) & 0x01) << 21);
        self.w
    }
}
#[doc = "Reader of field `LEDC_DUTY_CHNG_END_LSCH4_INT_ST`"]
pub type LEDC_DUTY_CHNG_END_LSCH4_INT_ST_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `LEDC_DUTY_CHNG_END_LSCH4_INT_ST`"]
pub struct LEDC_DUTY_CHNG_END_LSCH4_INT_ST_W<'a> {
    w: &'a mut W,
}
impl<'a> LEDC_DUTY_CHNG_END_LSCH4_INT_ST_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 20)) | (((value as u32) & 0x01) << 20);
        self.w
    }
}
#[doc = "Reader of field `LEDC_DUTY_CHNG_END_LSCH3_INT_ST`"]
pub type LEDC_DUTY_CHNG_END_LSCH3_INT_ST_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `LEDC_DUTY_CHNG_END_LSCH3_INT_ST`"]
pub struct LEDC_DUTY_CHNG_END_LSCH3_INT_ST_W<'a> {
    w: &'a mut W,
}
impl<'a> LEDC_DUTY_CHNG_END_LSCH3_INT_ST_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 19)) | (((value as u32) & 0x01) << 19);
        self.w
    }
}
#[doc = "Reader of field `LEDC_DUTY_CHNG_END_LSCH2_INT_ST`"]
pub type LEDC_DUTY_CHNG_END_LSCH2_INT_ST_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `LEDC_DUTY_CHNG_END_LSCH2_INT_ST`"]
pub struct LEDC_DUTY_CHNG_END_LSCH2_INT_ST_W<'a> {
    w: &'a mut W,
}
impl<'a> LEDC_DUTY_CHNG_END_LSCH2_INT_ST_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 18)) | (((value as u32) & 0x01) << 18);
        self.w
    }
}
#[doc = "Reader of field `LEDC_DUTY_CHNG_END_LSCH1_INT_ST`"]
pub type LEDC_DUTY_CHNG_END_LSCH1_INT_ST_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `LEDC_DUTY_CHNG_END_LSCH1_INT_ST`"]
pub struct LEDC_DUTY_CHNG_END_LSCH1_INT_ST_W<'a> {
    w: &'a mut W,
}
impl<'a> LEDC_DUTY_CHNG_END_LSCH1_INT_ST_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 17)) | (((value as u32) & 0x01) << 17);
        self.w
    }
}
#[doc = "Reader of field `LEDC_DUTY_CHNG_END_LSCH0_INT_ST`"]
pub type LEDC_DUTY_CHNG_END_LSCH0_INT_ST_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `LEDC_DUTY_CHNG_END_LSCH0_INT_ST`"]
pub struct LEDC_DUTY_CHNG_END_LSCH0_INT_ST_W<'a> {
    w: &'a mut W,
}
impl<'a> LEDC_DUTY_CHNG_END_LSCH0_INT_ST_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 16)) | (((value as u32) & 0x01) << 16);
        self.w
    }
}
#[doc = "Reader of field `LEDC_DUTY_CHNG_END_HSCH7_INT_ST`"]
pub type LEDC_DUTY_CHNG_END_HSCH7_INT_ST_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `LEDC_DUTY_CHNG_END_HSCH7_INT_ST`"]
pub struct LEDC_DUTY_CHNG_END_HSCH7_INT_ST_W<'a> {
    w: &'a mut W,
}
impl<'a> LEDC_DUTY_CHNG_END_HSCH7_INT_ST_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 15)) | (((value as u32) & 0x01) << 15);
        self.w
    }
}
#[doc = "Reader of field `LEDC_DUTY_CHNG_END_HSCH6_INT_ST`"]
pub type LEDC_DUTY_CHNG_END_HSCH6_INT_ST_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `LEDC_DUTY_CHNG_END_HSCH6_INT_ST`"]
pub struct LEDC_DUTY_CHNG_END_HSCH6_INT_ST_W<'a> {
    w: &'a mut W,
}
impl<'a> LEDC_DUTY_CHNG_END_HSCH6_INT_ST_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 14)) | (((value as u32) & 0x01) << 14);
        self.w
    }
}
#[doc = "Reader of field `LEDC_DUTY_CHNG_END_HSCH5_INT_ST`"]
pub type LEDC_DUTY_CHNG_END_HSCH5_INT_ST_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `LEDC_DUTY_CHNG_END_HSCH5_INT_ST`"]
pub struct LEDC_DUTY_CHNG_END_HSCH5_INT_ST_W<'a> {
    w: &'a mut W,
}
impl<'a> LEDC_DUTY_CHNG_END_HSCH5_INT_ST_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 13)) | (((value as u32) & 0x01) << 13);
        self.w
    }
}
#[doc = "Reader of field `LEDC_DUTY_CHNG_END_HSCH4_INT_ST`"]
pub type LEDC_DUTY_CHNG_END_HSCH4_INT_ST_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `LEDC_DUTY_CHNG_END_HSCH4_INT_ST`"]
pub struct LEDC_DUTY_CHNG_END_HSCH4_INT_ST_W<'a> {
    w: &'a mut W,
}
impl<'a> LEDC_DUTY_CHNG_END_HSCH4_INT_ST_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 12)) | (((value as u32) & 0x01) << 12);
        self.w
    }
}
#[doc = "Reader of field `LEDC_DUTY_CHNG_END_HSCH3_INT_ST`"]
pub type LEDC_DUTY_CHNG_END_HSCH3_INT_ST_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `LEDC_DUTY_CHNG_END_HSCH3_INT_ST`"]
pub struct LEDC_DUTY_CHNG_END_HSCH3_INT_ST_W<'a> {
    w: &'a mut W,
}
impl<'a> LEDC_DUTY_CHNG_END_HSCH3_INT_ST_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 11)) | (((value as u32) & 0x01) << 11);
        self.w
    }
}
#[doc = "Reader of field `LEDC_DUTY_CHNG_END_HSCH2_INT_ST`"]
pub type LEDC_DUTY_CHNG_END_HSCH2_INT_ST_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `LEDC_DUTY_CHNG_END_HSCH2_INT_ST`"]
pub struct LEDC_DUTY_CHNG_END_HSCH2_INT_ST_W<'a> {
    w: &'a mut W,
}
impl<'a> LEDC_DUTY_CHNG_END_HSCH2_INT_ST_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 10)) | (((value as u32) & 0x01) << 10);
        self.w
    }
}
#[doc = "Reader of field `LEDC_DUTY_CHNG_END_HSCH1_INT_ST`"]
pub type LEDC_DUTY_CHNG_END_HSCH1_INT_ST_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `LEDC_DUTY_CHNG_END_HSCH1_INT_ST`"]
pub struct LEDC_DUTY_CHNG_END_HSCH1_INT_ST_W<'a> {
    w: &'a mut W,
}
impl<'a> LEDC_DUTY_CHNG_END_HSCH1_INT_ST_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 9)) | (((value as u32) & 0x01) << 9);
        self.w
    }
}
#[doc = "Reader of field `LEDC_DUTY_CHNG_END_HSCH0_INT_ST`"]
pub type LEDC_DUTY_CHNG_END_HSCH0_INT_ST_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `LEDC_DUTY_CHNG_END_HSCH0_INT_ST`"]
pub struct LEDC_DUTY_CHNG_END_HSCH0_INT_ST_W<'a> {
    w: &'a mut W,
}
impl<'a> LEDC_DUTY_CHNG_END_HSCH0_INT_ST_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 8)) | (((value as u32) & 0x01) << 8);
        self.w
    }
}
#[doc = "Reader of field `LEDC_LSTIMER3_OVF_INT_ST`"]
pub type LEDC_LSTIMER3_OVF_INT_ST_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `LEDC_LSTIMER3_OVF_INT_ST`"]
pub struct LEDC_LSTIMER3_OVF_INT_ST_W<'a> {
    w: &'a mut W,
}
impl<'a> LEDC_LSTIMER3_OVF_INT_ST_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 7)) | (((value as u32) & 0x01) << 7);
        self.w
    }
}
#[doc = "Reader of field `LEDC_LSTIMER2_OVF_INT_ST`"]
pub type LEDC_LSTIMER2_OVF_INT_ST_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `LEDC_LSTIMER2_OVF_INT_ST`"]
pub struct LEDC_LSTIMER2_OVF_INT_ST_W<'a> {
    w: &'a mut W,
}
impl<'a> LEDC_LSTIMER2_OVF_INT_ST_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 6)) | (((value as u32) & 0x01) << 6);
        self.w
    }
}
#[doc = "Reader of field `LEDC_LSTIMER1_OVF_INT_ST`"]
pub type LEDC_LSTIMER1_OVF_INT_ST_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `LEDC_LSTIMER1_OVF_INT_ST`"]
pub struct LEDC_LSTIMER1_OVF_INT_ST_W<'a> {
    w: &'a mut W,
}
impl<'a> LEDC_LSTIMER1_OVF_INT_ST_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 5)) | (((value as u32) & 0x01) << 5);
        self.w
    }
}
#[doc = "Reader of field `LEDC_LSTIMER0_OVF_INT_ST`"]
pub type LEDC_LSTIMER0_OVF_INT_ST_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `LEDC_LSTIMER0_OVF_INT_ST`"]
pub struct LEDC_LSTIMER0_OVF_INT_ST_W<'a> {
    w: &'a mut W,
}
impl<'a> LEDC_LSTIMER0_OVF_INT_ST_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 4)) | (((value as u32) & 0x01) << 4);
        self.w
    }
}
#[doc = "Reader of field `LEDC_HSTIMER3_OVF_INT_ST`"]
pub type LEDC_HSTIMER3_OVF_INT_ST_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `LEDC_HSTIMER3_OVF_INT_ST`"]
pub struct LEDC_HSTIMER3_OVF_INT_ST_W<'a> {
    w: &'a mut W,
}
impl<'a> LEDC_HSTIMER3_OVF_INT_ST_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 3)) | (((value as u32) & 0x01) << 3);
        self.w
    }
}
#[doc = "Reader of field `LEDC_HSTIMER2_OVF_INT_ST`"]
pub type LEDC_HSTIMER2_OVF_INT_ST_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `LEDC_HSTIMER2_OVF_INT_ST`"]
pub struct LEDC_HSTIMER2_OVF_INT_ST_W<'a> {
    w: &'a mut W,
}
impl<'a> LEDC_HSTIMER2_OVF_INT_ST_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 2)) | (((value as u32) & 0x01) << 2);
        self.w
    }
}
#[doc = "Reader of field `LEDC_HSTIMER1_OVF_INT_ST`"]
pub type LEDC_HSTIMER1_OVF_INT_ST_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `LEDC_HSTIMER1_OVF_INT_ST`"]
pub struct LEDC_HSTIMER1_OVF_INT_ST_W<'a> {
    w: &'a mut W,
}
impl<'a> LEDC_HSTIMER1_OVF_INT_ST_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 1)) | (((value as u32) & 0x01) << 1);
        self.w
    }
}
#[doc = "Reader of field `LEDC_HSTIMER0_OVF_INT_ST`"]
pub type LEDC_HSTIMER0_OVF_INT_ST_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `LEDC_HSTIMER0_OVF_INT_ST`"]
pub struct LEDC_HSTIMER0_OVF_INT_ST_W<'a> {
    w: &'a mut W,
}
impl<'a> LEDC_HSTIMER0_OVF_INT_ST_W<'a> {
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
        self.w.bits = (self.w.bits & !0x01) | ((value as u32) & 0x01);
        self.w
    }
}
impl R {
    #[doc = "Bit 23 - The interrupt status bit for low speed channel 7 duty change done event"]
    #[inline(always)]
    pub fn ledc_duty_chng_end_lsch7_int_st(&self) -> LEDC_DUTY_CHNG_END_LSCH7_INT_ST_R {
        LEDC_DUTY_CHNG_END_LSCH7_INT_ST_R::new(((self.bits >> 23) & 0x01) != 0)
    }
    #[doc = "Bit 22 - The interrupt status bit for low speed channel 6 duty change done event."]
    #[inline(always)]
    pub fn ledc_duty_chng_end_lsch6_int_st(&self) -> LEDC_DUTY_CHNG_END_LSCH6_INT_ST_R {
        LEDC_DUTY_CHNG_END_LSCH6_INT_ST_R::new(((self.bits >> 22) & 0x01) != 0)
    }
    #[doc = "Bit 21 - The interrupt status bit for low speed channel 5 duty change done event."]
    #[inline(always)]
    pub fn ledc_duty_chng_end_lsch5_int_st(&self) -> LEDC_DUTY_CHNG_END_LSCH5_INT_ST_R {
        LEDC_DUTY_CHNG_END_LSCH5_INT_ST_R::new(((self.bits >> 21) & 0x01) != 0)
    }
    #[doc = "Bit 20 - The interrupt status bit for low speed channel 4 duty change done event."]
    #[inline(always)]
    pub fn ledc_duty_chng_end_lsch4_int_st(&self) -> LEDC_DUTY_CHNG_END_LSCH4_INT_ST_R {
        LEDC_DUTY_CHNG_END_LSCH4_INT_ST_R::new(((self.bits >> 20) & 0x01) != 0)
    }
    #[doc = "Bit 19 - The interrupt status bit for low speed channel 3 duty change done event."]
    #[inline(always)]
    pub fn ledc_duty_chng_end_lsch3_int_st(&self) -> LEDC_DUTY_CHNG_END_LSCH3_INT_ST_R {
        LEDC_DUTY_CHNG_END_LSCH3_INT_ST_R::new(((self.bits >> 19) & 0x01) != 0)
    }
    #[doc = "Bit 18 - The interrupt status bit for low speed channel 2 duty change done event."]
    #[inline(always)]
    pub fn ledc_duty_chng_end_lsch2_int_st(&self) -> LEDC_DUTY_CHNG_END_LSCH2_INT_ST_R {
        LEDC_DUTY_CHNG_END_LSCH2_INT_ST_R::new(((self.bits >> 18) & 0x01) != 0)
    }
    #[doc = "Bit 17 - The interrupt status bit for low speed channel 1 duty change done event."]
    #[inline(always)]
    pub fn ledc_duty_chng_end_lsch1_int_st(&self) -> LEDC_DUTY_CHNG_END_LSCH1_INT_ST_R {
        LEDC_DUTY_CHNG_END_LSCH1_INT_ST_R::new(((self.bits >> 17) & 0x01) != 0)
    }
    #[doc = "Bit 16 - The interrupt status bit for low speed channel 0 duty change done event."]
    #[inline(always)]
    pub fn ledc_duty_chng_end_lsch0_int_st(&self) -> LEDC_DUTY_CHNG_END_LSCH0_INT_ST_R {
        LEDC_DUTY_CHNG_END_LSCH0_INT_ST_R::new(((self.bits >> 16) & 0x01) != 0)
    }
    #[doc = "Bit 15 - The interrupt status bit for high speed channel 7 duty change done event."]
    #[inline(always)]
    pub fn ledc_duty_chng_end_hsch7_int_st(&self) -> LEDC_DUTY_CHNG_END_HSCH7_INT_ST_R {
        LEDC_DUTY_CHNG_END_HSCH7_INT_ST_R::new(((self.bits >> 15) & 0x01) != 0)
    }
    #[doc = "Bit 14 - The interrupt status bit for high speed channel 6 duty change done event."]
    #[inline(always)]
    pub fn ledc_duty_chng_end_hsch6_int_st(&self) -> LEDC_DUTY_CHNG_END_HSCH6_INT_ST_R {
        LEDC_DUTY_CHNG_END_HSCH6_INT_ST_R::new(((self.bits >> 14) & 0x01) != 0)
    }
    #[doc = "Bit 13 - The interrupt status bit for high speed channel 5 duty change done event."]
    #[inline(always)]
    pub fn ledc_duty_chng_end_hsch5_int_st(&self) -> LEDC_DUTY_CHNG_END_HSCH5_INT_ST_R {
        LEDC_DUTY_CHNG_END_HSCH5_INT_ST_R::new(((self.bits >> 13) & 0x01) != 0)
    }
    #[doc = "Bit 12 - The interrupt status bit for high speed channel 4 duty change done event."]
    #[inline(always)]
    pub fn ledc_duty_chng_end_hsch4_int_st(&self) -> LEDC_DUTY_CHNG_END_HSCH4_INT_ST_R {
        LEDC_DUTY_CHNG_END_HSCH4_INT_ST_R::new(((self.bits >> 12) & 0x01) != 0)
    }
    #[doc = "Bit 11 - The interrupt status bit for high speed channel 3 duty change done event."]
    #[inline(always)]
    pub fn ledc_duty_chng_end_hsch3_int_st(&self) -> LEDC_DUTY_CHNG_END_HSCH3_INT_ST_R {
        LEDC_DUTY_CHNG_END_HSCH3_INT_ST_R::new(((self.bits >> 11) & 0x01) != 0)
    }
    #[doc = "Bit 10 - The interrupt status bit for high speed channel 2 duty change done event."]
    #[inline(always)]
    pub fn ledc_duty_chng_end_hsch2_int_st(&self) -> LEDC_DUTY_CHNG_END_HSCH2_INT_ST_R {
        LEDC_DUTY_CHNG_END_HSCH2_INT_ST_R::new(((self.bits >> 10) & 0x01) != 0)
    }
    #[doc = "Bit 9 - The interrupt status bit for high speed channel 1 duty change done event."]
    #[inline(always)]
    pub fn ledc_duty_chng_end_hsch1_int_st(&self) -> LEDC_DUTY_CHNG_END_HSCH1_INT_ST_R {
        LEDC_DUTY_CHNG_END_HSCH1_INT_ST_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 8 - The interrupt status bit for high speed channel 0 duty change done event."]
    #[inline(always)]
    pub fn ledc_duty_chng_end_hsch0_int_st(&self) -> LEDC_DUTY_CHNG_END_HSCH0_INT_ST_R {
        LEDC_DUTY_CHNG_END_HSCH0_INT_ST_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 7 - The interrupt status bit for low speed channel3 counter overflow event."]
    #[inline(always)]
    pub fn ledc_lstimer3_ovf_int_st(&self) -> LEDC_LSTIMER3_OVF_INT_ST_R {
        LEDC_LSTIMER3_OVF_INT_ST_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bit 6 - The interrupt status bit for low speed channel2 counter overflow event."]
    #[inline(always)]
    pub fn ledc_lstimer2_ovf_int_st(&self) -> LEDC_LSTIMER2_OVF_INT_ST_R {
        LEDC_LSTIMER2_OVF_INT_ST_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 5 - The interrupt status bit for low speed channel1 counter overflow event."]
    #[inline(always)]
    pub fn ledc_lstimer1_ovf_int_st(&self) -> LEDC_LSTIMER1_OVF_INT_ST_R {
        LEDC_LSTIMER1_OVF_INT_ST_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 4 - The interrupt status bit for low speed channel0 counter overflow event."]
    #[inline(always)]
    pub fn ledc_lstimer0_ovf_int_st(&self) -> LEDC_LSTIMER0_OVF_INT_ST_R {
        LEDC_LSTIMER0_OVF_INT_ST_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 3 - The interrupt status bit for high speed channel3 counter overflow event."]
    #[inline(always)]
    pub fn ledc_hstimer3_ovf_int_st(&self) -> LEDC_HSTIMER3_OVF_INT_ST_R {
        LEDC_HSTIMER3_OVF_INT_ST_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 2 - The interrupt status bit for high speed channel2 counter overflow event."]
    #[inline(always)]
    pub fn ledc_hstimer2_ovf_int_st(&self) -> LEDC_HSTIMER2_OVF_INT_ST_R {
        LEDC_HSTIMER2_OVF_INT_ST_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 1 - The interrupt status bit for high speed channel1 counter overflow event."]
    #[inline(always)]
    pub fn ledc_hstimer1_ovf_int_st(&self) -> LEDC_HSTIMER1_OVF_INT_ST_R {
        LEDC_HSTIMER1_OVF_INT_ST_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 0 - The interrupt status bit for high speed channel0 counter overflow event."]
    #[inline(always)]
    pub fn ledc_hstimer0_ovf_int_st(&self) -> LEDC_HSTIMER0_OVF_INT_ST_R {
        LEDC_HSTIMER0_OVF_INT_ST_R::new((self.bits & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 23 - The interrupt status bit for low speed channel 7 duty change done event"]
    #[inline(always)]
    pub fn ledc_duty_chng_end_lsch7_int_st(&mut self) -> LEDC_DUTY_CHNG_END_LSCH7_INT_ST_W {
        LEDC_DUTY_CHNG_END_LSCH7_INT_ST_W { w: self }
    }
    #[doc = "Bit 22 - The interrupt status bit for low speed channel 6 duty change done event."]
    #[inline(always)]
    pub fn ledc_duty_chng_end_lsch6_int_st(&mut self) -> LEDC_DUTY_CHNG_END_LSCH6_INT_ST_W {
        LEDC_DUTY_CHNG_END_LSCH6_INT_ST_W { w: self }
    }
    #[doc = "Bit 21 - The interrupt status bit for low speed channel 5 duty change done event."]
    #[inline(always)]
    pub fn ledc_duty_chng_end_lsch5_int_st(&mut self) -> LEDC_DUTY_CHNG_END_LSCH5_INT_ST_W {
        LEDC_DUTY_CHNG_END_LSCH5_INT_ST_W { w: self }
    }
    #[doc = "Bit 20 - The interrupt status bit for low speed channel 4 duty change done event."]
    #[inline(always)]
    pub fn ledc_duty_chng_end_lsch4_int_st(&mut self) -> LEDC_DUTY_CHNG_END_LSCH4_INT_ST_W {
        LEDC_DUTY_CHNG_END_LSCH4_INT_ST_W { w: self }
    }
    #[doc = "Bit 19 - The interrupt status bit for low speed channel 3 duty change done event."]
    #[inline(always)]
    pub fn ledc_duty_chng_end_lsch3_int_st(&mut self) -> LEDC_DUTY_CHNG_END_LSCH3_INT_ST_W {
        LEDC_DUTY_CHNG_END_LSCH3_INT_ST_W { w: self }
    }
    #[doc = "Bit 18 - The interrupt status bit for low speed channel 2 duty change done event."]
    #[inline(always)]
    pub fn ledc_duty_chng_end_lsch2_int_st(&mut self) -> LEDC_DUTY_CHNG_END_LSCH2_INT_ST_W {
        LEDC_DUTY_CHNG_END_LSCH2_INT_ST_W { w: self }
    }
    #[doc = "Bit 17 - The interrupt status bit for low speed channel 1 duty change done event."]
    #[inline(always)]
    pub fn ledc_duty_chng_end_lsch1_int_st(&mut self) -> LEDC_DUTY_CHNG_END_LSCH1_INT_ST_W {
        LEDC_DUTY_CHNG_END_LSCH1_INT_ST_W { w: self }
    }
    #[doc = "Bit 16 - The interrupt status bit for low speed channel 0 duty change done event."]
    #[inline(always)]
    pub fn ledc_duty_chng_end_lsch0_int_st(&mut self) -> LEDC_DUTY_CHNG_END_LSCH0_INT_ST_W {
        LEDC_DUTY_CHNG_END_LSCH0_INT_ST_W { w: self }
    }
    #[doc = "Bit 15 - The interrupt status bit for high speed channel 7 duty change done event."]
    #[inline(always)]
    pub fn ledc_duty_chng_end_hsch7_int_st(&mut self) -> LEDC_DUTY_CHNG_END_HSCH7_INT_ST_W {
        LEDC_DUTY_CHNG_END_HSCH7_INT_ST_W { w: self }
    }
    #[doc = "Bit 14 - The interrupt status bit for high speed channel 6 duty change done event."]
    #[inline(always)]
    pub fn ledc_duty_chng_end_hsch6_int_st(&mut self) -> LEDC_DUTY_CHNG_END_HSCH6_INT_ST_W {
        LEDC_DUTY_CHNG_END_HSCH6_INT_ST_W { w: self }
    }
    #[doc = "Bit 13 - The interrupt status bit for high speed channel 5 duty change done event."]
    #[inline(always)]
    pub fn ledc_duty_chng_end_hsch5_int_st(&mut self) -> LEDC_DUTY_CHNG_END_HSCH5_INT_ST_W {
        LEDC_DUTY_CHNG_END_HSCH5_INT_ST_W { w: self }
    }
    #[doc = "Bit 12 - The interrupt status bit for high speed channel 4 duty change done event."]
    #[inline(always)]
    pub fn ledc_duty_chng_end_hsch4_int_st(&mut self) -> LEDC_DUTY_CHNG_END_HSCH4_INT_ST_W {
        LEDC_DUTY_CHNG_END_HSCH4_INT_ST_W { w: self }
    }
    #[doc = "Bit 11 - The interrupt status bit for high speed channel 3 duty change done event."]
    #[inline(always)]
    pub fn ledc_duty_chng_end_hsch3_int_st(&mut self) -> LEDC_DUTY_CHNG_END_HSCH3_INT_ST_W {
        LEDC_DUTY_CHNG_END_HSCH3_INT_ST_W { w: self }
    }
    #[doc = "Bit 10 - The interrupt status bit for high speed channel 2 duty change done event."]
    #[inline(always)]
    pub fn ledc_duty_chng_end_hsch2_int_st(&mut self) -> LEDC_DUTY_CHNG_END_HSCH2_INT_ST_W {
        LEDC_DUTY_CHNG_END_HSCH2_INT_ST_W { w: self }
    }
    #[doc = "Bit 9 - The interrupt status bit for high speed channel 1 duty change done event."]
    #[inline(always)]
    pub fn ledc_duty_chng_end_hsch1_int_st(&mut self) -> LEDC_DUTY_CHNG_END_HSCH1_INT_ST_W {
        LEDC_DUTY_CHNG_END_HSCH1_INT_ST_W { w: self }
    }
    #[doc = "Bit 8 - The interrupt status bit for high speed channel 0 duty change done event."]
    #[inline(always)]
    pub fn ledc_duty_chng_end_hsch0_int_st(&mut self) -> LEDC_DUTY_CHNG_END_HSCH0_INT_ST_W {
        LEDC_DUTY_CHNG_END_HSCH0_INT_ST_W { w: self }
    }
    #[doc = "Bit 7 - The interrupt status bit for low speed channel3 counter overflow event."]
    #[inline(always)]
    pub fn ledc_lstimer3_ovf_int_st(&mut self) -> LEDC_LSTIMER3_OVF_INT_ST_W {
        LEDC_LSTIMER3_OVF_INT_ST_W { w: self }
    }
    #[doc = "Bit 6 - The interrupt status bit for low speed channel2 counter overflow event."]
    #[inline(always)]
    pub fn ledc_lstimer2_ovf_int_st(&mut self) -> LEDC_LSTIMER2_OVF_INT_ST_W {
        LEDC_LSTIMER2_OVF_INT_ST_W { w: self }
    }
    #[doc = "Bit 5 - The interrupt status bit for low speed channel1 counter overflow event."]
    #[inline(always)]
    pub fn ledc_lstimer1_ovf_int_st(&mut self) -> LEDC_LSTIMER1_OVF_INT_ST_W {
        LEDC_LSTIMER1_OVF_INT_ST_W { w: self }
    }
    #[doc = "Bit 4 - The interrupt status bit for low speed channel0 counter overflow event."]
    #[inline(always)]
    pub fn ledc_lstimer0_ovf_int_st(&mut self) -> LEDC_LSTIMER0_OVF_INT_ST_W {
        LEDC_LSTIMER0_OVF_INT_ST_W { w: self }
    }
    #[doc = "Bit 3 - The interrupt status bit for high speed channel3 counter overflow event."]
    #[inline(always)]
    pub fn ledc_hstimer3_ovf_int_st(&mut self) -> LEDC_HSTIMER3_OVF_INT_ST_W {
        LEDC_HSTIMER3_OVF_INT_ST_W { w: self }
    }
    #[doc = "Bit 2 - The interrupt status bit for high speed channel2 counter overflow event."]
    #[inline(always)]
    pub fn ledc_hstimer2_ovf_int_st(&mut self) -> LEDC_HSTIMER2_OVF_INT_ST_W {
        LEDC_HSTIMER2_OVF_INT_ST_W { w: self }
    }
    #[doc = "Bit 1 - The interrupt status bit for high speed channel1 counter overflow event."]
    #[inline(always)]
    pub fn ledc_hstimer1_ovf_int_st(&mut self) -> LEDC_HSTIMER1_OVF_INT_ST_W {
        LEDC_HSTIMER1_OVF_INT_ST_W { w: self }
    }
    #[doc = "Bit 0 - The interrupt status bit for high speed channel0 counter overflow event."]
    #[inline(always)]
    pub fn ledc_hstimer0_ovf_int_st(&mut self) -> LEDC_HSTIMER0_OVF_INT_ST_W {
        LEDC_HSTIMER0_OVF_INT_ST_W { w: self }
    }
}

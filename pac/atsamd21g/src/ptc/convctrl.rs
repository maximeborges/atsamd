#[doc = "Register `CONVCTRL` reader"]
pub type R = crate::R<ConvctrlSpec>;
#[doc = "Register `CONVCTRL` writer"]
pub type W = crate::W<ConvctrlSpec>;
#[doc = "ADC Accumulator\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Oversamplecount {
    #[doc = "0: 1 sample per report"]
    Oversample1 = 0,
    #[doc = "1: 2 samples per report"]
    Oversample2 = 1,
    #[doc = "2: 4 samples per report"]
    Oversample4 = 2,
    #[doc = "3: 8 samples per report"]
    Oversample8 = 3,
    #[doc = "4: 16 samples per report"]
    Oversample16 = 4,
    #[doc = "5: 32 samples per report"]
    Oversample32 = 5,
    #[doc = "6: 64 samples per report"]
    Oversample64 = 6,
}
impl From<Oversamplecount> for u8 {
    #[inline(always)]
    fn from(variant: Oversamplecount) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Oversamplecount {
    type Ux = u8;
}
impl crate::IsEnum for Oversamplecount {}
#[doc = "Field `ADCACCUM` reader - ADC Accumulator"]
pub type AdcaccumR = crate::FieldReader<Oversamplecount>;
impl AdcaccumR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Oversamplecount> {
        match self.bits {
            0 => Some(Oversamplecount::Oversample1),
            1 => Some(Oversamplecount::Oversample2),
            2 => Some(Oversamplecount::Oversample4),
            3 => Some(Oversamplecount::Oversample8),
            4 => Some(Oversamplecount::Oversample16),
            5 => Some(Oversamplecount::Oversample32),
            6 => Some(Oversamplecount::Oversample64),
            _ => None,
        }
    }
    #[doc = "1 sample per report"]
    #[inline(always)]
    pub fn is_oversample1(&self) -> bool {
        *self == Oversamplecount::Oversample1
    }
    #[doc = "2 samples per report"]
    #[inline(always)]
    pub fn is_oversample2(&self) -> bool {
        *self == Oversamplecount::Oversample2
    }
    #[doc = "4 samples per report"]
    #[inline(always)]
    pub fn is_oversample4(&self) -> bool {
        *self == Oversamplecount::Oversample4
    }
    #[doc = "8 samples per report"]
    #[inline(always)]
    pub fn is_oversample8(&self) -> bool {
        *self == Oversamplecount::Oversample8
    }
    #[doc = "16 samples per report"]
    #[inline(always)]
    pub fn is_oversample16(&self) -> bool {
        *self == Oversamplecount::Oversample16
    }
    #[doc = "32 samples per report"]
    #[inline(always)]
    pub fn is_oversample32(&self) -> bool {
        *self == Oversamplecount::Oversample32
    }
    #[doc = "64 samples per report"]
    #[inline(always)]
    pub fn is_oversample64(&self) -> bool {
        *self == Oversamplecount::Oversample64
    }
}
#[doc = "Field `ADCACCUM` writer - ADC Accumulator"]
pub type AdcaccumW<'a, REG> = crate::FieldWriter<'a, REG, 3, Oversamplecount>;
impl<'a, REG> AdcaccumW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "1 sample per report"]
    #[inline(always)]
    pub fn oversample1(self) -> &'a mut crate::W<REG> {
        self.variant(Oversamplecount::Oversample1)
    }
    #[doc = "2 samples per report"]
    #[inline(always)]
    pub fn oversample2(self) -> &'a mut crate::W<REG> {
        self.variant(Oversamplecount::Oversample2)
    }
    #[doc = "4 samples per report"]
    #[inline(always)]
    pub fn oversample4(self) -> &'a mut crate::W<REG> {
        self.variant(Oversamplecount::Oversample4)
    }
    #[doc = "8 samples per report"]
    #[inline(always)]
    pub fn oversample8(self) -> &'a mut crate::W<REG> {
        self.variant(Oversamplecount::Oversample8)
    }
    #[doc = "16 samples per report"]
    #[inline(always)]
    pub fn oversample16(self) -> &'a mut crate::W<REG> {
        self.variant(Oversamplecount::Oversample16)
    }
    #[doc = "32 samples per report"]
    #[inline(always)]
    pub fn oversample32(self) -> &'a mut crate::W<REG> {
        self.variant(Oversamplecount::Oversample32)
    }
    #[doc = "64 samples per report"]
    #[inline(always)]
    pub fn oversample64(self) -> &'a mut crate::W<REG> {
        self.variant(Oversamplecount::Oversample64)
    }
}
#[doc = "Field `CONVERT` reader - Start conversion"]
pub type ConvertR = crate::BitReader;
#[doc = "Field `CONVERT` writer - Start conversion"]
pub type ConvertW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:2 - ADC Accumulator"]
    #[inline(always)]
    pub fn adcaccum(&self) -> AdcaccumR {
        AdcaccumR::new(self.bits & 7)
    }
    #[doc = "Bit 7 - Start conversion"]
    #[inline(always)]
    pub fn convert(&self) -> ConvertR {
        ConvertR::new(((self.bits >> 7) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:2 - ADC Accumulator"]
    #[inline(always)]
    #[must_use]
    pub fn adcaccum(&mut self) -> AdcaccumW<ConvctrlSpec> {
        AdcaccumW::new(self, 0)
    }
    #[doc = "Bit 7 - Start conversion"]
    #[inline(always)]
    #[must_use]
    pub fn convert(&mut self) -> ConvertW<ConvctrlSpec> {
        ConvertW::new(self, 7)
    }
}
#[doc = "Conversion control\n\nYou can [`read`](crate::Reg::read) this register and get [`convctrl::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`convctrl::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ConvctrlSpec;
impl crate::RegisterSpec for ConvctrlSpec {
    type Ux = u8;
}
#[doc = "`read()` method returns [`convctrl::R`](R) reader structure"]
impl crate::Readable for ConvctrlSpec {}
#[doc = "`write(|w| ..)` method takes [`convctrl::W`](W) writer structure"]
impl crate::Writable for ConvctrlSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
}
#[doc = "`reset()` method sets CONVCTRL to value 0"]
impl crate::Resettable for ConvctrlSpec {
    const RESET_VALUE: u8 = 0;
}

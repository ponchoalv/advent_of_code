use crate::year_2017::day_1_2017::day_1_2017;
use crate::year_2017::day_2_2017::{day_2_2017, day_2_2_2017};
use crate::year_2017::day_3_2017::{day_3_1_2017, get_distance_for_number, get_distance_with_spiral_struct, day_3_2_2017};
use crate::year_2017::day_4_2017::{day_4_1_2017, day_4_2_2017};
use crate::year_2017::day_5_2017::{day_5_1_2017, day_5_2_2017};
use crate::year_2017::day_6_2017::{day_6_1_2017, day_6_2_2017};
use crate::year_2017::day_7_2017::day_7_1_2017;

pub fn run_advent_of_code_2017() {
    println!("--- Advent of Code 2017 ---");

    let day_1_part_1_vals = "77736991856689225253142335214746294932318813454849177823468674346512426482777696993348135287531487622845155339235443718798255411492778415157351753377959586612882455464736285648473397681163729345143319577258292849619491486748832944425643737899293811819448271546283914592546989275992844383947572926628695617661344293284789225493932487897149244685921644561896799491668147588536732985476538413354195246785378443492137893161362862587297219368699689318441563683292683855151652394244688119527728613756153348584975372656877565662527436152551476175644428333449297581939357656843784849965764796365272113837436618857363585783813291999774718355479485961244782148994281845717611589612672436243788252212252489833952785291284935439662751339273847424621193587955284885915987692812313251556836958571335334281322495251889724281863765636441971178795365413267178792118544937392522893132283573129821178591214594778712292228515169348771198167462495988252456944269678515277886142827218825358561772588377998394984947946121983115158951297156321289231481348126998584455974277123213413359859659339792627742476688827577318285573236187838749444212666293172899385531383551142896847178342163129883523694183388123567744916752899386265368245342587281521723872555392212596227684414269667696229995976182762587281829533181925696289733325513618571116199419759821597197636415243789757789129824537812428338192536462468554399548893532588928486825398895911533744671691387494516395641555683144968644717265849634943691721391779987198764147667349266877149238695714118982841721323853294642175381514347345237721288281254828745122878268792661867994785585131534136646954347165597315643658739688567246339618795777125767432162928257331951255792438831957359141651634491912746875748363394329848227391812251812842263277229514125426682179711184717737714178235995431465217547759282779499842892993556918977773236196185348965713241211365895519697294982523166196268941976859987925578945185217127344619169353395993198368185217391883839449331638641744279836858188235296951745922667612379649453277174224722894599153367373494255388826855322712652812127873536473277";
    println!(
        "    * Day 1 - Part 1 ==> {}",
        day_1_2017(day_1_part_1_vals, 1)
    );

    let day_1_part_2_vals = "77736991856689225253142335214746294932318813454849177823468674346512426482777696993348135287531487622845155339235443718798255411492778415157351753377959586612882455464736285648473397681163729345143319577258292849619491486748832944425643737899293811819448271546283914592546989275992844383947572926628695617661344293284789225493932487897149244685921644561896799491668147588536732985476538413354195246785378443492137893161362862587297219368699689318441563683292683855151652394244688119527728613756153348584975372656877565662527436152551476175644428333449297581939357656843784849965764796365272113837436618857363585783813291999774718355479485961244782148994281845717611589612672436243788252212252489833952785291284935439662751339273847424621193587955284885915987692812313251556836958571335334281322495251889724281863765636441971178795365413267178792118544937392522893132283573129821178591214594778712292228515169348771198167462495988252456944269678515277886142827218825358561772588377998394984947946121983115158951297156321289231481348126998584455974277123213413359859659339792627742476688827577318285573236187838749444212666293172899385531383551142896847178342163129883523694183388123567744916752899386265368245342587281521723872555392212596227684414269667696229995976182762587281829533181925696289733325513618571116199419759821597197636415243789757789129824537812428338192536462468554399548893532588928486825398895911533744671691387494516395641555683144968644717265849634943691721391779987198764147667349266877149238695714118982841721323853294642175381514347345237721288281254828745122878268792661867994785585131534136646954347165597315643658739688567246339618795777125767432162928257331951255792438831957359141651634491912746875748363394329848227391812251812842263277229514125426682179711184717737714178235995431465217547759282779499842892993556918977773236196185348965713241211365895519697294982523166196268941976859987925578945185217127344619169353395993198368185217391883839449331638641744279836858188235296951745922667612379649453277174224722894599153367373494255388826855322712652812127873536473277";

    println!(
        "    * Day 1 - Part 2 ==> {}",
        day_1_2017(day_1_part_2_vals, day_1_part_2_vals.len() / 2)
    );

    let val = "790	99	345	1080	32	143	1085	984	553	98	123	97	197	886	125	947
302	463	59	58	55	87	508	54	472	63	469	419	424	331	337	72
899	962	77	1127	62	530	78	880	129	1014	93	148	239	288	357	424
2417	2755	254	3886	5336	3655	5798	3273	5016	178	270	6511	223	5391	1342	2377
68	3002	3307	166	275	1989	1611	364	157	144	3771	1267	3188	3149	156	3454
1088	1261	21	1063	1173	278	1164	207	237	1230	1185	431	232	660	195	1246
49	1100	136	1491	647	1486	112	1278	53	1564	1147	1068	809	1638	138	117
158	3216	1972	2646	3181	785	2937	365	611	1977	1199	2972	201	2432	186	160
244	86	61	38	58	71	243	52	245	264	209	265	308	80	126	129
1317	792	74	111	1721	252	1082	1881	1349	94	891	1458	331	1691	89	1724
3798	202	3140	3468	1486	2073	3872	3190	3481	3760	2876	182	2772	226	3753	188
2272	6876	6759	218	272	4095	4712	6244	4889	2037	234	223	6858	3499	2358	439
792	230	886	824	762	895	99	799	94	110	747	635	91	406	89	157
2074	237	1668	1961	170	2292	2079	1371	1909	221	2039	1022	193	2195	1395	2123
8447	203	1806	6777	278	2850	1232	6369	398	235	212	992	7520	7304	7852	520
3928	107	3406	123	2111	2749	223	125	134	146	3875	1357	508	1534	4002	4417";

    println!("    * Day 2 - Part 1 ==> {:?}", day_2_2017(val));

    println!("    * Day 2 - Part 2 ==> {:?}", day_2_2_2017(val));

    println!("    * Day 3 - Part 1 ==> {}", day_3_1_2017(361_527));

    println!(
        "    * Day 3 - Part 1 (with spiral iter) ==> {}",
        get_distance_for_number(361_527)
    );

    println!(
        "    * Day 3 - Part 1 (with spiral struct iter impl) ==> {}",
        get_distance_with_spiral_struct(361_527)
    );

    println!("    * Day 3 - Part 2 ==> {}", day_3_2_2017(361_527));

    let input = "pphsv ojtou brvhsj cer ntfhlra udeh ccgtyzc zoyzmh jum lugbnk
vxjnf fzqitnj uyfck blnl impo kxoow nngd worcm bdesehw
caibh nfuk kfnu llfdbz uxjty yxjut jcea
qiho qif eupwww avyglnj nxzotsu hio lws
xjty usocjsh pivk qnknunc yjcgh bwya djw zpyr
ycfmfe mgq sjiomg nfzjul bjwkmgu yvsnvgj dcjupu wzz blmn
rdowgbt vpwfdoi blzl laghnk gsa vhnpo cztxzlb rtz hvwonhb eciju pfjtbo
bqs bqs dbutvgf mmzb izpyud rap izpyud xlzeb mnj hjncs
xpu vwp nujcos piu irindir tpmfd umtvlm gznu
sfpuxar qcnbte omouazv cnh uaxspfr sepolf rusafpx
xbmaf iceyqqq sabpt gliexel muubepe qqiyqce fmrcc eazk obkeonl fmccr kgk
apg gbycwe gap pag
gagv saqbk lwtllc wnhzz khxsjc
lgc alen rlmsp anel gcbvg
bujlaz rks rlqf deknmee yrp
scqvl weusbc bgvaz vgg cjwsfno vqy zbq aqy tvf bgzav
hbki vei fxdwljs myjuba elbsib pvy xxjxgi dtgv
linzaeu qbwdke fdg pykw
qvtdd aco aav bpu mvkcuc kjfj japgfki jfdl gem hog bdzsiea
wpbigkb lzhwba jssjkn qvb kmwu qddv
iny osyvqnt tumunzb torq bdeneg wywank poza ipp iggorw
tuko mhdbsf vmjdop jomaqpj rcdsud hmgspr lsas nzmwc
cirkjq nmjuu xtgejv gtexvj vjcmtqq unjmu
xsdmezq xvqjvqp exhygy qahju hvd qadmdh lok
wvvys kax rohrrar rwhnvi lhnmefp lsktouy bxilosp
wayf diobnl zvu obnidl oibnld
cewil ygsf ffzp ruxhu vah lnvwt aef lnnjc kgkb gxtlx feko
uti epphrin pywths cpzzh csjei nczhamy gayxmb bdcytq xkx fgmt
qvzyuwi dwo swkw bwjdrn dasgd ijgw vzabaop yefyhmc wgij
dyg sugrf vid etz weyqg nyntx dwfgwm khon hnzzzn xfyra
ofbh bdrsk rdrjj elaxvk jrjdr
msxau rsocvx zxdda mxz lknl
qktaywx dirpdbf unqnd wbrwkuu fvmqwl emxr big
xwz kvsydc ayokjyy qiah omw neo htltxx fxhwqwj colqvbb sxmo ephfkex
ncjxoaf fwjkc czmhv ylg axcjofn dvj bzqjku opvcr jiwzucg vmhzc
gmmnrt zqar twdwrg qiwwki fcbr lixm hjdwwe moiva
roinlxg cxeezve whannk cxeezve pyoj boweioy cpkgxsz
qkct qso xlb xyy aellfet rzt cbboow devfb nih fhbfxzi
qyc ltxia alixt atilx xtgrv
svruz ufvo rvesnxv dik vzurs jjg idk
xeudhrg hudn cilo ljplosb
kpb oyzvywx vldko qhfkwod bkeutk zqcqug pbriu wqocos
qkngzfy whobyri aze jvipdty ocirbep icqwc
kzxxlab sjr zhymws xkbx
nnxs gkwtld dwhkry snuibq dtdl aicug bhtlfzp qzk jctos
regvro mxcq hqof yraucxi jhkol iuxineo pbtnk rfjwc szgjpr ndqqj vfgm
yqrfox xoqrfy utbryu utubyr
jdubjt wqrl wnk rlqw nwiq pnbn qinw uaff ftdo htfrav
rum mur umr tij ovbahl losao imawwpb wadhww tbteyqc
napxd kzeiqcp ppgqucm xkityt frq hugrp gjgtt gmuqppc zwqme
xyuzs ysch howlzgu dkqppbs nvbiz mks mtxv vivouex uvawq
ffe lfsn nlq mpulheq ikcfo wdtz cnwsbph zkib muu
bqkxav wtecb lxwdhr kqbavx aqxvbk
czwswqx ldkxapd pfwd bdkkj iqohla cwosw ihqpd pcc ckhabbn
foiip hau rbqiyhh htm omeubgh symh evfcqg
lqx xlq rsgf izu esetis
npsrkdj fvulgkw eovw mzr uobcze azb tij ihoer ehori jit wknsqhm
gnrksh xwggt oosi bpnmhx qqaa mpmryu jhzyz
yad gbexqcr gbexqcr gbexqcr
ldca xxhznn twyy ytwy zhxnnx xfmpi
floioot kfyh dhibv ezyznar sfg sfg ezyznar
cinioim iiocmin ypla aypl
mhwcjbz dftuqsy wswop eizbf ptsd
ehx mlh nfxgfkz uuw xftmn ptlkbo vsnyo ttwce
oexvf orcg cncnkfk comvhl
lqewsj lyulrcl efixd qvd fhznqnz yvrkwyi xmhgc vzbp
dmr wrxqh thcm giomp rtvl ssc gwq rbklw hcmt fjvud
teozhb dmzwfv qkq pvcqfqq
hvlebc qqmg repxk zwrjdx ztruwb such tyligs ybg
psa rqznokd lgc jstqres yiqt mbiody xazb xjuk dtb
lea ncm rnh myzqzwm
wjml eums ueflvbr cjpgnl qduunu zfxaai jwlm lprzzg vrn ttetyr sume
uwkgeu uiahd plyewgi vveo nwhsitz mcitc uvk zsxehgs sewl
lnbdrka sgtivn sozzq mgd vhxfnlr twrfpk
gadphmk mbx lmlbrf tsnehnr lawdpm fnima gxgl
umty vrn dpow fsnnpjv fsnvnjp nnsvpjf cioaio
euu uue zeskmtk hob stekkzm
ypqpri qwdju ypriqp iprqyp jnoxqa
lkppi ingfxw wlulvp yhwrli nxwigf oyuhq ggfslx
kdd ypvr pyvr waw vyrp khqq mamxca bapq gobfm
iuq upvdpv zxef bfwns lmq lxswr kpsqo pwde iaaou nsw udy
lgzo nil ovgrmt omgtrv jrqp pqrj lit
uumyu iiakfj gvdtzz qbux yxn ejs dvlts
hcm ghutxq zswi tmyhqef hgxtuq
shkhkdk kad seubeax kdl mzu
cpykgr skx rfhpor xsk moyhlai ogv ophfrr dxipuuh
beyw jvrre opodn zdoajhx fhg ijs drczy drczy hjungq
jrzieja gfg yzdn yxm wshibsn fgg
xtylh vxscmvp rfymq uzhpyea spxcmvv dlni msj yxhlt
eov awql miv miv eov
mmvrfbg fjiyf hvqz zpuqmbf fszyuz ldfgni wemfjl fjjpl rbnpy rfb
ppzpeh nam ntv xnchtyk hja hpepzp foj bibvx nmmdlff bsrkp
qiy qiy umhlnh qiy
tyds oepk wae tdsy sdty
ukawr rkwau ghtjhm axy
wtbjiv btjivw ewaf hwk ttq
kdpun myve sqv rhvpy fnjwt puw ujhf thsp nkdadqr
vyw wkkpdpy xlgz lmmcuve ncuq lmotk
pmsfw vxd jpe qxlyasx ejp gwuv
pmgyndm ezofbvx nicbwrw kwnlj yjvnas fdpkfo mqcsyhn pyjpf fbexvzo vkftm erl
trmwvk rywuzoz hbidea kicohfz heidab deaibh
sogf govd dknpk vxrvk rlm vwhjk
xnxbfmw wguzrhd zbmkz piwppa mkbzz xvwrdgy flusfqb
cgduq hbnwr xfx mrejb ckw zkbaihf cloow cwk wuvthv iwqctx
vugx qbucd gxuv ocb cob
ilmet fbelxxz qratdfn unoj hbc duv srmikz
vnzuw zgpbqgf uzm thysyxd dinfh bgvr olungg ksd dsetwqz hpg
omagsf zpr coa kknx bzithq pewp flvoz xiiq weojqr wpep
aagj gcglqt gqcglt xbfx dhdx lbx
pljq plxuscw ilh wfk lhi hli fouieyw
hvnh zvm aqy dzitirm veq ctux
lglhs aqibdii hjbn cfgc qrg pnbntcx owoks ebz
jozngde lwne mbo omb fnyzvvj gndozje
bbdgc igtdj uhahgp sqduko
uuspedu fgnspm ewc slly jbs chl heanm abqijx kadvgxu
akfsft skna kusjqr rkqujs
erc vrljpu lruvjp lpvjur
iors hcdr fsqtcj vop vmn dtqnz tov oscjlw cdrh ctfjsq lrnts
fxp mczo sjlcxa mzoc jmsq hcxybow dmrr bcoxhyw
aac ewraerq odmxpz aac aac
zzio zebmxa szeej poordr gmi owwnnh xfx rzrab lfey jesze
akc yyoj vqod drtne
joxhvyf ymasnbr omouvq isxdrr
qyi ayrkzu jsk vqvvno jkkuxi zufnnwu mrsszdf
ocqi htfb tzjna cdt wkzhynm eergf
yokzugl usyuqu qvotq uweqyow lygkzuo kpmqmb uglyzok
glvshl imqv jrv xlpnsy gcg psj irtiamg wkl
bjcpc nvyloa dkkan efj okubpc cxlowm eone kmpny
cyxqys nmuaftv gqxj gtvsc
beouh dioxiah kizdy hyi cozrray rave fqxmxmj gdm
frjz amrsat lxvhzj azhevtu vxlzhj
zwmnrk sbk txzrcsj sbk oosgfej cvh zuthibi onvwd sbk nhwpzq
gzamt vraw kuk ugayl lyaug bww rwav ijah
bdjirxg vifjr rhbxpa oao yrhjxoi pbn
navb umesiys yhix phuhu aekkciu nlnsiq wjf idqdwp
cmhw rsu urs ziprlfe
kyhxitv cgty bnwjyq cygt sgjn pdab imarvhg yjbnqw
axaa ejancv yau njpc jvwy bpft kwjvg qzrbvtm diu njpc bpft
ambj upe rmqr yudbiqf krudp pqyf
tnb mobnpv vep ohxoc cyip wxyccfo jrbi rwsws kls zlv oohxc
fjh dmb hlbq bqc jhf kax suz fjjg rkpc
wjnn byfirm goeyh xtjmdka
tgyfxx hefpxln mveobqr yeo ftfn srt vim vlcu hevoi xtaaff
imyql xotcl poql rlueapq bkwykm hlalk bkwykm
gkec zff hbmtq rjxjbcf arerlu pvz cdaqi nijmhv uodwjh
mpctof mopftc ksfbat sbkatf
nvdd jub bvi kyggdbx nwtiok gjt mgsm dbhsn rzibgjm dvdn eqi
ysd iirp dfgzza wiyeoou ysd ispkv bcqg wwzqgq xphse
ntq ivposb gsd ezl tlkztp lez qyurp vxsmg dgs
wijs rydbj onm usiyqzb hwrol giusanb kewukl yziuqbs doojam nom
lfacyy xwwast truqtt tzneimn uxsydc ktu eqyaj ndszak
ffleooc kikif fohgop aucy moubqxu
iaxc pnwexdl ncy vmwm xrqoi wpgftq rofx utyzjuf stdxq twpgfq
ppmlp etsvi cjdx poly ynx vfxpslg mqjo qnpsage flpsxvg jwsxiqt
lbyhnb kflrpeq ssoti webxr embbjd kbnx ubzqco
khhc vwuqzb ebocbko rwmonkz edfqn hzh qhncoq gbwdi wjeg ocwow
ghzhd kcxblp lzwkkr gzhdh umk pblcxk
wyajtw jiff ouylv sni lwhlrg avqjiis igzx wbl lhrwgl
glhh kaxha tqii hwzx rgic kaxha rgyidmt qdgxfl ynjc oibfij
bapj bix rjniw ynbql idlvnmt wynpzbl zlpuoix kvn kakwys
aldpxxu iojxp rif xbyqtr jffdvy qnrq tqwsdiu
ulssco ktbymjw bfj zhkg zgc ctyri
ilrmq wfahcgk mrlqi bguad inj
cjzc rekuy ifr wfkg sple
cvjkp qbmumnp mprg ltmwxxh zpemtyb ozzssfd ksu mgrp
nvc sxp mpkxz bhlctq hguaa yrdkm iwsgfg qjssh gobbies hucdh
jdxrjw qmo qmo vobhnu
dnjib wtjp rfdjqdj skpvrb vkwevb kxxovp
fzi kicta zkuvr rfaawv ehklq cfdjsyb tukahwr zkuvr kicta ouq
aba ytdguk gqmpn hvxabff hvxabff dckj
fna wxyqhxd hvy khsu yypoyy lvvue medheua gim slf drdbeh ikihf
jquz wwo wwo ghlz jrbvb jrbvb
jwzvkl yjw ouwla yjw ouwla
zsvlgyf rzqbtj qygynem ukdgjm lbsyh tmdzp fbcaim eymzr
pvw sbs dvsa plmepl pwv ayxk vpw dwt
inayadn pnti yzhxk azga gxq aznbciu gjnmyqm
isgf ndqmk beyqq ebyqq srtzxo aiiw oqfuwp uoqwfp buejctv pxbk
pzl irv tzvzdb wcy eszm ybwiw ycw riizifd iybww
btpu cua azzqffy owcr
ofwq sqlpzat lozdxlc aevjmpc lcolzxd wbbysn qwfo vcrx gdzgi
dbpfmxu ydsxwl ijn svxtop csep ldqeog ffye zcrl soh aclw
wyiyyhv vyhiywy obgi hiyywvy
ddvaoc lhv spurn rgxyy onjw illvn yryxg xyyrg
vid wdttqq kajr myip
wolqlue phlunpt dcmmkfm sgxk dmmckmf sfng jlbsntq dxp
zmneyho fswj xdgsjc oefwjdi htgxvbd tgqrq xodoa
ynw bygqdnh hhmnkuw cojqrke qszzdjo orskwq mdfae asabn
vvpm vkj pcxghao caoxphg axhblxb vvmp
txox nzy eqn zgir dytsi girz ffa ugjjbzj brob fll
kbz pukqbd fiwmuh umwihf bkz dvz
vgs vejs vejs vejs mbkyjjy
viqmnmu bitkyw nddnk dknnd cldnpp hipub plcdpn fdzzpb mmyomn
ndylnfx gozlrx ngptk rnpteb wtacx xmtcjy xldha
fey doyxis ampmtr ycqh syw cqhlj hnngx
dijf nac tvkq ayo akbj lzmngdm wfxpn bpyvrf cvdqpa
zsofz lhho hgat wqskga mnt
mylwm zxsd omzpa waz hcrr lxmpq jsw sqtwak pzoma
rwhgsgt ysdq ztihici mpwcawv alkqg wsxiwx
snldn bcb anjdv cbb awsscc cqxult hjmjew mcycb fdpdg sesrh
kukrqm fawafz qdim wyobtqx bnvjnqg dcvqxta yptr nnpu ughldqp duo zafwaf
knb yjqb bscpnt nzg sqeu zkahna ttuf nsbtpc ixwit vucwj idix
bfqyx xlnpc ijrxu zkqi kjxtahr fgag orusms adi bfqyx bfqyx
dqddc ncbv bvfk hefikb dqddc hqjl otpx zfiu
ntkv qunrzx eztzure ctt rjo bkdt znvd jwdf gqhf mmhrzgt
zeavm hkbf rawqwuf pis dojlkt vnjhmi uvk cufmn qginezd xyut
hnidzk chlctc yst pepd dxntbxg vqk daxfpmu wshyddl
jgd vesqgo bdyqy igl ahstdm wjtd lrtkjsv tjsj sccxbih esn gkkzj
iisiswh jll rhlaf jqwwgfa wmhyo izva vrg zjkak nlxxfer rvhx
mkrkd jlqtpy ukstro ktuors wsj ynqpbp kpiyxzv nxeiwg xpzvkiy
jbr gnct fwklekg cmfqnm ctn gqobrs kwht
pztmjs yiffc kfhsblx yiffc yiffc
biezil iiezbl bzeiil smocoju
viiigm gmmmk yeiv dxzogro qsmzsur hukzwjn lcle syo mdj uruf rxfseu
extchsd adeff ouikoj fyaclr rwwvqsd dooe tcxheds zrdqqhm fdoxv kbxi tlcj
aycnydq qlxhka zoi shplo qll
bfry lbwckm ltq rbfy gpn vojp ruj dpxcve geq
svtvfwh lca lac qia vhwsftv nookdfz xgjiaf yvcdlt
aspgqym fryuzhx bbydf tbn bwutsc fqgi zij lmxhog qnmse
rbb gsys volnas onvlas lonasv vwjdso lnteapy
got iauk kficn jvfuy yvoe jcxwui hyamqx mke mwh jcxwui hyamqx
avutfi ggmha dkopc kothnnb syoi xsd wjedywy
oziejyz yzeijoz hnthyn knj juuq qujtp kgq bymlnlf yicf
zsejuy dybeap hvowmvn okxb yoi epadby cnzjk xfwprzc
lacg iiix fblhxvf nrkkol lnafzw qspzsn gvdy ipj zub uouseo
evukwkh ycjxxc lptwmf pmd izxdsos zrkavf pgjoy zwokg mpjiej
vqw ijwoy eaw wvq svmcq ccxi nyub ynlq eqornax uprt pygfe
plue okbbm btvm gba kutn jacjx ysqt lvx pcxxu qcf
pyw ffjfudq bvk hsdwdva fjnivhf odbmw krpgrj
hziesm bxa dceiwt tmvivjk snl fkh dahsxyx kqlhak lurtk
xss sswyxrg yqff dbkx kbxd mpzbmnl bzplnmm
uvz pjm ilrol pmj uzct ztcu brhkv
heiz jcn syjt zfvlvaq aflvqvz amcjh rxnitw
cxl nxvrn vjnz aewtr cxtko nnvcp ltptd adpxt zvjn fntklj
aymmm tuirj hzngq zhbh paqs kvpfo aqsp kmo acprw sabrso kdqmp
ndqjspv mmhp pndjsvq rti usm
ije oad mvelyg jadz ekm dao zdcmv
qwww tmwmdbb oxxfoza rgmf eonku brh gcgiuoi ojscn
fjedeek ohlax fiydku rbnxpg wfivg cdgs
axwbni hojye mwfe oyqknxp whdgfy ihku mbhr gagnz hehagxj
hibautd blnayq lnayqb gepml mgpel qunw
ircx oeb kujtip zbu ebo cmmn
upyqvot wbponp hnn vav avv tvrky omm
yzqsnf agbfsw dbxoya sfnqzy hqrxek qsnyzf oagyerm xxhukm
xzvk mvcwz oujr hell hoe xexa dqlpqt xdqz ucola hsvv tcmybhl
skldxr mzyol ybzyzd jnnxb rxncdy nkpwy fwlnsw omylz oiwieu fshv ngvha
jkwqf yxrox hejfoq orxyx
rijken xiwf mawqcfu erinjk jsi yyg mmu mdkfqb
ornjes krp eornjs enjros pyqp nnwwjl
wzd uqqo kyeli tikdle aykdjog uiz rbpnw mjxezf ihiz rlgyg
cjm ajqgvkz kfgyy dmczlc mjc kxcm zctyqgh ymsk jwhqfd czpqgan
vxkzvco owo qogj uyictoj kfr pyoo ejrru npluynx bvv jhhzu kuciwc
eqk pcsly kelu arzgoe trfo fotr cuaax
lagonw qvcssqz sdoklh uvovi sfrkmd hnvafj ltg wfjj
viwbkm hpwe kzzwrbr axjtlq mznin wwpjg unlwur
nuzorgo qfoz ydisca qxdfutv hzg
nqgge tobtt hjocx ntyqyi rxzkynw wrnxzyk ciscy trjt ottbt
yuii srawx gljxe eteogz kcu jlgxe tjik ktsnp agudqok jwol vfnyv
vgicg dhnrmxz sjhozy hlalx rutwq
nyoyoje kco hoyam hoyam tta iflud amh gdxcsj vqr fvsqcgv
xdmbtph ueen cskerl rxjvpdc
nricn addljzg obq rikez igq bxygkmv qmgojou uheubk qor
snzd ztusvr vrstzu mceddga hgu
vvrbfjg mcdhmsf ldtwl otuna gmjurrx jgrurxm rxmurjg yrioq
iotkvo sftfvn vvoit lllju xvlg rdsb ywmdf mzxigu kzq
sgqw gqsw lqfu wgqs xpiwou jurgucd azq wgaqpm
ijntzi chlnfj yjqatz hjflcn vys ofq oqf oadthe jrfw
mmc motjo vcwmod rpaszfk zgkkua bpja vjb htrk
bpfvvka kmger mnvvfl hakudy yfprdoo mvnlfv rgmek evnwg
mykpu juavkn cecdvi aszbi lxm hmps oaqoif
fshizd fsdzhi lvcq hhpb eavwno auqlwz rpv owcdojx amsmf qgnddd
pohmcn hlcxk qsesxh rncr
fgyrsis ldem avxmnh frpodq oefzn
plfpu qdyojz xdrzrjy kpv abkh fge bbnotvp liikmcu czvwl oyh
ovha muitw pzy edfjoo fhsxuh dliyruc dikcd cqem ywfy
exyry jtzqn tscr qbtxno cikk poqgr tnjzq eofe sxea anlikep kick
zcie purpw dmhhms bcdo prwup uprpw wfejgjd
kwtjc cmixp dodfwj hcgmmat pkeyspo ubnl ajxvj ffkh xvw
nvlgq oduus psufiqg lrwpn dleftn xtllqvf usgz
liarf sczsf sczsf wky qtzq qvve qvve
cit vtjsh jrhkyvi txj urmq hppx
rhblmxn rhblmxn lkgow dylurwc beyk gfcewxj ehpl disoe tjbjy lkgow
nbkrm jvk ffux ars agns bebic jzjfm kmnbr gptvtsa ufxf
hrlvup jaz tafyr qcgq wkd fiz bgsrx jmtcvo qkbvj
eontk djf tiafrng mtwat puainel nyjoh meynxbf eqdw
aspvmbx tgzuszm fpj xkl nzpr fjp vnomk byx sbtov tnu utn
ldyww gwmiddv hwyh gcgsdit gtgdisc suufl xsw dlwyw
sye dgbd wyf ixqzthx dgdb esy
nsdgera fqz xwbdgui ngdgbcd bcn qrdxml cwcmxws tncm mqsodj cqgk
estayas cocmbpv cdcf vygtswo aplwa estayas
ndc ndc wntr sfls sfls
gse svv esmi lcdii lnr kemrk gnk ildic blnqy wvn
mwlpm awkr sxsudub yauwww hnktbog fpnqc nmxoq yoparu tqjpkug nbipft
czwnkk hrodtmx yyzpil ooqjb cvxzfh
kwa wak gipak gsgrw
jyy fja jjk kuvoqdy urqx
doyu chgn gvtxi qjdigvy kxr dizwrjc sll zenl yyblj
epxeqih kfi hlog pakk kkiidrh hiufw wuhif baqzxzi bgcd phi jzjdxjp
hllhyad sodc nyrtfe kygof hyyqi txddqg wcwxvnt ewqmj wwv
vxymuoe caat diqwbo vfruxdf sqniefn hetcbl nvtttu ouesb
yvoez pvthzc tdowuci wjijicn fhpmq kfobag yctdwj
xaugkb rprkg tidpx pjk tpwwm pbcfhr wmwpt sfynrl iouaw zbnyu
auakc culuxg bffg rodyhea ixlmtfb jdurl szoa
xgona fjzho buh khbvti ddh mgj ptgaqps
dqldupd udpldqd poku gfgpcg zsvk grvk kntx jih uwvxdvq sivk
mwdnq wmqdn uzto mdqnw
alvfm qxqo thwru xqqo jilnsgs rnonk fwntuby ogbha
gvxlxyf cdpv khvpka kgt gshlaa tenb
mtgvvxh mrjrsd truk rrerzx tujweaz
ozepw gsqkr rtmmc cmrtm
spnthg xhlzuu xwcrxz aqqejhh bpzh
ectdftk rgp mkp vxp pevriz wkgfkaw vfygj peg gep wjn
bksbu ywsszf tsbrps vxicr hfustju ynnlbo
sio urbvf ujezjk vkyc ukjezj bvrfu qwwgqmw uqfekvx bzipxus qfumwh
druru kycweog ycmef rjyy fkgp
rmf ifbip rsztco coju wlr bfbmsug lwr bsufbgm nwmp
jjuxtyd yif rkldsvu binq spepa mfg aszm
ghilaau ncm sgbavz omzeotz azukf bgjw zqzo gjbw pld
gtog iqheik budeu guvljmi
qqlj jqql ttk xcxu
cfq cfq kpagib dxfxufw hhksbjh gpcp
xkeax acnia jjubfc mhot uxlhh gnkj pavta rciondm rkquh xudqian
wqhqzg psqh rnnc uujlgq
hpjpaoa maa rdndl xewqj nmagwx xewqj hxuyvou xziv rdndl fbxmbz hmfwghy
dtwnrca hbfcptw qrmvat sdatx les zwizogq
bodiwzg sgoas fsf wgkrn zgbdowi wfkz
ngcsg grtao wcfxpyl gngcs fxwycpl fkpt
txvngo vxngot tkoap zqjc qzcj oeruix myh ihgdfik qtt
rxeh fcbnoo rxeh lve wvoc pmnxej dlcbrh rztt noibg
zyvq lwxqu oyjv bvidmf wxuql
wzc zcw czw dnhkvrg nzslrf
cfgl uwhxu qnsfmt tgyabes mqnq nkitq hmcvxlt qqmn yzmb uomqp
lwziur hgmdmv zuvipkp vir apr gfaq zeo dunat mqgafzg
prq pqkr xlrw njf ncqni kgpoma cmtklv
jwfuc poz opz fuple
fgleub lcgnifu lkwo kftbc onvwvdx lukpod xgmh rnj
rwqvv ezjmoni llq ekd cdvv kzcci gzsj vuipv fnw
rtnua gbnzg kqtogns iozzwc kjpzz kiiurey yzlvzx cpy xrue
fexcjmw ebwssx ewbcgwd uwolou nfdhic vupiykn jss djoo xftbkgo
idf ipvmez qyevwd wfsjxja dif dig
szpbtsa bssaztp sptzasb qppgz odur cpmn wpmg
pxn zjmq rbnr azwstzm mln upaqyty nxp oge nlm
bfaryqv hag phtvh ypi
epeeog lip zqio wuehlnb bau sbd dsb
xbrrp sej agrqnpa aarpnqg bnwyi jbn
uqmsvd asmuyy czxviw pznnmvc
sddwmek wnaea iwphupk sabo
cingdks ksh mtyip zltgafm dflkcd wbdnqup uokm gmxpyd libz svv akce
qge ewv dkabkmb xcpi nrkmsu mkmb djvamg mhhrwjh
krjt etfhm bxzatw zdkvz ehov seyxbw mkiirs plzoplu sogmwb wodfcle
qwea adibdp emo homrd pjcrhlc eqaw kqsrp rphjlcc
gajzo nwjg qxjra jztcnir ijvjwez avxb afz zyywqz kcszgh elmlkfh
lbz ozia bctf bumoji anhil rta xvit
ejybire ypjl qevak fzalx mlh qxlei zib
xmzas kwojjz ntrnrw nbmxlv mdgxs xjhxg suo zdcrxl qkujisz pxmu
eezyd unrtm wyu vhufvto rpb isfcy ygh hgy
nszvbzv ebtt memrsva ebtt qwcaq bhbas pvzfbov ppjbdy nszvbzv jabvrp
rlo zbmi lugvu yeby
tfcd tvl faaq mnural nyarh xnxk ctdf bodz
vwdrhc gub bgu fpcovx rcvwhd jukwsue
aekrhi lpknnrh bett tkib ioqrap igwnst aekrhi lhha
acg mknhazp pcgjuk tajplv
masq fyjkn agq qhxbbl qga npzj fme xtihic rntisg iqv aqg
ipagh fjth mswztpi iexd cocojy vhqrla joe wrsrmw
njztu tsh auqrxca zpp
jctn webxi haq irrr qox irrr webxi
reaw axmnvd voakf lnz ftbxfh zjyxzl pryfjpv sistgb pov mshs
gsy ctsngl ptmnyx vpjx zpvtori pfu ioycdrq
aobdtlj osdnrth sgqe geqs qegs
oamrlxk ygbb rkamoxl nztl sarbmtj yqupjt plu sbtarmj vpa rxea
yvhgp yznko epwpza gqrsod rilukp cglhomj wnaplu ugvdko qdr
cggztg ajw gggzct ubmiefj kpa
rel lvasbh kobm mdnzla pwnyj ehep gzx nhjdnsg rxa
qaz gook rplqwh vsht
dhe aneq ivrn awekad ckcbt zsqca ehd rvni oulwfuu
oxgzzow wntz tkqaoi oxgzzow lwkdpgy lhd aekjasp tkqaoi dnhaw
alxghco cpanoa onjh hyeyebe whxn zfu zozbll gojn
zdqulsa dlqsazu zqudals sfedw
rydtrsv rrtvysd fvyza drdgh lsfzt blnxr cnxe tslzf iijyds ylcxn
cczea nxx kwol kopaza wuvr cyvoo whlicv
zbmrwdq tlzbevx jwzpsc uvkwpd bmss rbzblj
jogx jgi gji hypmtkg ijg oscjv
flkoqja kwmrqv wzehel fvmcfap mkwqvr ivwxg jqfwdvo hweezl
vgjg nzucho nuohcz ggvj tmxci
fqaqx zeybhtg bxeic lftuqp wzuerz sww qfltxk
keiy myrvp blkxcg lncqmsu diittlg fqrf digrel cpwrk ipan dkxb bymlzo
owm irygdz pyhj mow wmo
noul pbvvt zcv ueqyjl zhetlw lpjfhli
felvwb wdykz kyibdz haq qkouj vuav oztyqh
dyxo njcr hcuk ysrr pucw qbajztc
ooyaz pmt hqwu gjx tmp tpm pwz
lyhzajz dfot avyifo kdwka pwypcep kyyw tirlku zdpjmft
aexle hfxo dacwvcy xsiotyg cifq ibupshj aktt rzvf pgafj
pxubhw ibpm jxtxg iwnssf osbpj
exmtfyx blbfg emrunru zkuhoi lfzn zrj unmcece phuppi
icomb rmy mvsqqkh zwjubz lumq wekx
cmdgs gsr pfhqx pfhqx cmdgs pga
rpyf jejc adaiou dutv imbenyu dqw zhebjhu pryf vtxs yprf
cxj roprjn rqoh qacagru snxd
rczvi hfpl luc yowgj nvavlhw vjudkmv dwu teq
klwc cktzh ksnvswl nsgeu xyohp mhs fxnjhm fwrcg rdeadkx cim
ounvb vzqje ujctzzk iyy vxck ebtvbqr uswsmcr jveqz qejzv jmi pboq
lwffygh mqsh vnnj ufz qhms gqfuxo lurzmu
buf psdluck gapwoo wgll sbfavbc lljfvzx cdgo rpt sfvabcb
svefr kubbri fervs nboi zkvq
jwr vtc zkcpzb kczbzp cdned pzbzkc wigjuak fszgweu odflfek
vwdqm khnnj plokjg vnce venc vecn yzxtgb
tawl yrhoz tawl yrhoz
vvehsl kdhzgme rix rcs btm pxnlsps vlhesv sxpnslp yqjtool
eqpyw kpmkcyw wqhglxg ajfzo hbd qvmhy nhokah iisqvad kxuyd fxek
jsz txhwhah hxt djnvl srylveu pxp dzmmn epek tzs
joyzql jqczueb rtdyw fyc fjirfyn tjcalz joyzql fyc
pjrmiz xwnmwns kcqjuut zfgxhdr octwn kqppg zhfgxrd wmwnnxs
ema yqxqs aljjo ajloj wozb
urgmhiz epqj vhhaxdm ptlsvig qzbmm cumbho lkg gyzmg eaopyzf ncfy mqe
ijvwvo oszkees ugvyk hjdj ftip itfp
ylfw qutzdj mgqp cyjss yzsdqqi iykvs fyor sthyqp mrjtzee hgo zwqbtgk
bkfkns gco bykzc mje dwmkrwt ljegqor yxjxp oaleuu
xeltq ggyqis aud frtyxhx iwz wiz fwoxz fozxw
zdu nwduqsa nced iphaaxo
bqjj oah ezd brhgxrc pmkz kdog exw
ihatt hck iepn egemprp wrz wzcuo xjzeaa wku ivjvihh
cwkuof bmj qmxd qbtms zgdei bsqmt ssndhw eeenku lcsqy bvvodr
tek zsgytci vgoun kwwu
jcxvp ijxc buqgix uil zfoku
ggndshq bmjeo yqaxtik blspz yofh edaroy
ipvtxh ouye elln dllvx iqza nhwf zyfw pvlky
iydcx gvarm gvarm wegmiy
sfjd liiflle mulboe qywzs tzbns trojl pad mnfcrhb sltb
gthqj jvpsof jwlfyeg jwhlfj
qckv umzrge gnzc mnr xde
gvgxmhv txnait taxint ius iboqdj
vsfex kbpvsby qembkb efxvs vhflzvm eaazg dyg bbmekq
wxpfk xwfpk xwkpf cjsyi
knzg eefq feqe seppop ttxz qnqfn atgsy cch mkjlbwt uyhct
quzw jbiw miqehe qvf jyipqh kzcjxyh
teuvzf tdtwoi pcuafa cwgjk ccur lgmqv jpjdkk efrnw uloqn dpkjkj lwloeph
yaffjy xntstsv gygq sxttvsn tvnstxs
cvbmdf pfrfkna wupv van iocb hsiyke obspj ytyfkl hbsqtij hkcw
oeddmnu koso mdodeun ybe mhjbmwy ubejz soko yxvuv
nylhy ylnyh olb vcdik
gsp ilba llnu jjk urbvuma qzypf bkceotg ezxq hyvjngf
tfnegyq rue waeif tfnegyq mvqm
wvgnsk cpd oib wrdfaz kohwgkc kzzig hogkwck gkizz
fecuuyp yfq bvanvxb cjeqwf unw dccr qzh zqu voakj
utoazh bjuq kmhcre izmny mirorsy twnl jyoc
fnnpd dmr ccgu eqgewc zuqivf
kkxiba qdabuen oikaz dnuywmm
aogud adugo uzcglpj lucv dgoua mdsqa mvrg
lymhv sof hvyml mlvhy nit
chu bwxp xpbw ghaix seklnc ola zofnrwt uch
wtt abob vblijtd oabb qjws
uozrpw kgf gxidxm uehdr fta pqakkrq atf fat woaolk
gaee voshd ghlyy emvzlkg cmcgk tuwlsj jwtsul znrta mjieqph glker
qiugxas gkg cbzmoz kahs obzzcm
puz omcokz gjc heuqb
dgndhb wid wdi scwnrjf juaisgo eivaw hgdndb
mgcrd hnqg pkpeb vprxcp
atlcnzp fyp cpkivxi bzj ypf cqpt bysu
pnd jiitmzs csw mxnpck vxutdrs ivipzy cws xiegsy qut
txlk avcvbuu hnq yyriq ajyswd urgiwc
qgiqut gvblizs giqnfrk tty mvoj wpikl giqnfrk bkdpndu xztmxn hsmqxf
llthg zjslki wilj rcyfois bavz hrqxn
ytbw hlkl vip skycogy ejiirhx
ndmtg bthlbw lsoq cvlvo sqol sqlo bppl sdkbls dtpyzrq vgm
psm xpj xjp lqi spm gqirw aglpj
htg fcchvyt xffev szdu lieadft
nbjo qohgzu vofg vvild dbtyi pdolxn plnoao jxze xlpbxj brajzg
urpp jjv lihmvp ivkwdqr sesyp ypbry qok sesyp ivkwdqr was
yinepzv qvnzdtf apv ucxo bdioo juga hjfsyl hmowo avc
dmiv tplae iiuiaxx tpale pyzkc
giwhst mpexd byfyc swuzkc
yydkwp xuu vjya kav ujmcxy qrtp zvlk
lsvdyn tkw qxu omvlc wwmfvov mrgcoov dhpu tfair hupd zbx njzgwtw
zuz rsxc xsrc gdwwf nycsv zzu kcu
unlvzv jerqqgm nozma ykbflj qihqkx
pctffo begf ivrvy ezru mvqt waocq
tubtuk gxkc ikgw bjrird kxjebbh sbjyc yafkd khqajmt aclpmf gqfo yrpf
rdt vrxa fyudo myeosb ursflwk
wbjras edlbwdp ctobtw jbvtvcd xjgoo cmunxm mjtbpi klovx bypmsab unc
xckml uztr htublq vilabvr jdiwus qejxur evfw qqm
tzqq tzqq wkb wkb
dgmg ljzc dgmg mbmco cgze qsap jccvot uors iiq
rwvac woylk dmn teorprx nyuvz hcwwxlj lvej drbjo asjgq
ljen tpfl vixcivr guaf lnje waim jlen
djgaa janhi adudm yzv zkcb xqw fgvrz
kpkjoon ggzx skp rqcsw xgzg zgxg jtf ghc
rtnyxo qixfd nphekk mouzk gny fpzquw qgywx rpr gqydze
gawdlv vrivoof rte iyp gaih sfzplm
csojx wzojode uzy qulr lylmb guvtkwv
ovxj aamms ftxo ebckdqw wqvsdci jwfqxks jafrcrn yyomrot
qnu jqwr ywudxk qpsez rdc kiyfz iiecf dthxjjb bown
typ zxcvjo rip acjhl paaab qhqipg xkguye sbxy pomkvn
ofvaegv hgak oafevgv hkemar rqkha grklnsp msvkkku rekahm bxmjnw
ahoihju sdyn phi uhz lupbx
lavt jef klmq oqyfpf kis nazul ymezxek xpla fxyrfnt
nwnagwy hvpjqfg sgm ungfstr gso owqqxjh
hey hye ipyrt qxmthg jth wpbr hxgmtq cvfkfux qykdzhk movcfnl vxyoc
zsras abnrj fgaczuk ssazr xzf cnxu gns wnqqy dwjh szars
uhb zanlvh lvdotkb xekl kcofo
lhx iccy ibkjw ciykxaj imsx ehamqlz iwzapxc rhaltv
pofit owmpqej vwrobh jvox gdqehss yyxd styu tfkm fiotp
ecz mdpoqsv mdpoqsv yxx rexok hcfll yvury hdhcfu juhkvpt rspnfj hxvgdir
ohed mtigaoe eodh agmiteo
vjvv hfco cppbxtw hawsjxz ovlsq qgs risgwhg auhj
togivgg czrtvw ccz wzvtrc bse lsk
ndc ndc lrfi iyleol nchx jxpv xdcsfmp nnx wtvq pih tgc
hzpf sur zhfp klfmhx lbuidp xiqimnf
qddpdk trfxpip pnsowj hidgvnf prur rsrautp aamykfm fysqjmq xwzjane mbmtxhf oqctt
lfd eops govslp ultbye vrqai hcjkcf snpape
cbok koumkad otpozb pqcs emilpe wpcyvxd bock
spjb xkkak anuvk ejoklh nyerw bsjp zxuq vcwitnd xxtjmjg zfgq xkpf
juo pmiyoh xxk myphio ogfyf dovlmwm moevao qqxidn
";

    println!("    * Day 4 - Part 1 ==> {}", day_4_1_2017(input));

    println!("    * Day 4 - Part 2 ==> {}", day_4_2_2017(input));

    let input = "2
0
0
1
2
0
1
-4
-5
0
-1
0
-6
0
-5
2
-9
-11
-15
-3
-11
-12
-14
-5
-16
-3
-13
-6
0
-3
-17
0
-17
-5
-1
-26
-21
-14
-20
-7
-24
-26
-32
-41
-2
-18
-18
-13
-28
0
-32
-3
-2
-14
-17
-54
-22
-34
-33
-34
0
-46
-3
-44
-58
-10
-56
-65
-46
-24
-20
-4
-27
-41
-33
-31
-20
-75
-73
-41
-36
-31
-70
-46
-1
-79
-61
-51
-77
-44
-55
-2
-18
-26
-50
-79
-59
-69
-62
-80
-13
-69
-97
-71
-24
-7
-40
-10
-23
-85
-97
-103
-55
-90
-40
-60
-98
-95
-39
-76
-63
-12
-2
-65
-109
-47
-12
-37
-5
-23
-125
-124
-49
-91
-70
-134
-54
-122
-135
-12
-23
-22
-83
-40
-133
-77
-88
-119
-146
-26
-12
-108
-63
-111
-148
-99
-77
-77
-76
-89
-37
-95
-105
-76
-137
-151
-146
-141
-162
-12
-68
-36
-116
-60
-73
-61
-60
-101
-168
-142
-143
-118
-165
-108
-179
-180
-11
-152
-67
-33
-10
-169
-155
-16
-136
-165
-164
2
1
-28
-131
-86
-153
-116
-113
-149
-66
-64
-36
-168
-116
-159
-15
-180
-125
-146
-135
-105
-161
-133
-207
-192
-192
-99
-146
-93
-21
-5
-189
-86
-16
-4
-44
-167
-20
-201
-110
-103
-223
-182
-71
-194
-68
-90
-237
-147
2
-88
-184
-90
-12
-119
-85
-138
-179
-152
-158
-82
-122
-179
-191
-120
-174
-165
-137
-181
-58
-250
-66
-194
-202
-171
-179
-137
-250
-248
-167
-108
-27
-175
-34
-254
-35
-157
-158
-31
-52
-236
-238
-247
-279
-209
-257
-167
-151
-7
-182
-2
-149
-87
-245
-141
-238
-186
-71
-97
-128
-147
-52
-93
-142
-197
-296
-73
-89
-14
-253
-190
-295
-312
-47
-236
-233
-238
-305
-121
-191
-251
-91
-307
-77
-228
-300
-197
-91
-191
-299
-77
-255
-51
-232
-64
-198
-187
-96
-86
-203
-216
-203
-343
-203
-78
-99
-174
-269
-349
-173
-52
-233
-154
-151
-304
-70
-235
-106
-226
-325
-142
-192
-115
-170
-15
-35
-174
-267
-108
-374
-128
-60
-131
-364
-371
-56
-96
-365
-305
-140
-50
-220
-179
-43
-356
-120
-216
-276
-103
-389
-28
-393
-341
-74
-85
-361
-68
-111
-4
-216
-263
-115
-194
-382
-285
-176
-145
-24
-59
-291
-170
-358
-226
-355
-292
-185
-297
-156
-248
-332
-319
-311
-46
-170
-428
-222
-35
-136
-206
-81
-330
-89
-75
-248
-42
-52
-24
-39
-129
-228
-242
-396
-222
-433
-168
-362
-4
-345
-395
-435
-14
-439
-136
-267
-417
-107
-177
-8
-208
-219
-222
-453
-155
-183
-252
0
-173
-71
-164
-187
-80
-292
-246
-89
-217
-227
-93
-244
-82
-51
-23
-283
-261
-50
-384
-415
-149
-103
-481
-404
-267
-80
-61
-130
-228
-310
-319
-186
-88
-173
-40
-69
-231
-398
-342
-176
-33
-304
-232
-220
-381
-436
-74
-116
-398
-467
-341
-483
-137
-5
-437
-67
-296
-137
-166
-216
-192
-307
-68
-319
-296
-524
-308
-68
-21
-515
-531
-221
-173
-261
-200
-450
-95
-366
-14
-29
-23
-173
-397
-373
-283
-104
-246
-153
-240
-378
-306
-495
-518
-459
-459
-340
-475
-96
-347
-8
-365
-7
-482
-113
-223
-313
-456
-89
-205
-507
-538
-115
-310
-484
-96
-367
-582
-32
-550
-247
-257
-479
-165
-346
-514
-188
-180
-506
-117
-92
-128
-507
-387
-52
-535
-210
-221
-560
-245
-70
-552
-99
-529
-607
-263
-345
-253
-426
-351
-92
-489
-478
-226
-606
-287
-277
-432
-336
-418
-94
-2
-192
-600
-454
-26
-3
-630
-294
-105
-439
-589
-425
-623
-451
-487
-117
-538
-78
-126
-485
-326
-455
-370
-389
-379
-158
-441
-524
-435
-160
-198
-172
-313
-380
-128
-166
-562
-427
-23
-616
-95
-188
-417
-419
-589
-488
-377
-520
-412
-348
-359
-488
-108
-409
-56
-460
-364
-233
-352
-59
-313
-609
-534
-432
-104
-514
-68
-83
-305
-308
-645
-535
-624
-453
-630
-274
-98
-280
-38
-443
-620
-411
-624
-379
-373
-338
-410
-382
-171
-645
-430
-294
-696
-513
-659
-690
-558
-2
-325
-234
-437
-610
-158
-186
-539
-405
-78
-100
-311
-201
-558
-604
-386
-457
-125
-419
-680
-147
-237
-107
-155
-550
-565
-214
-528
-353
-637
-6
-634
-332
-92
-474
-289
-617
-141
-398
-367
-537
-369
-88
-608
-699
-257
-602
-276
-406
-92
-746
-398
-387
-234
-331
-225
-480
-667
-264
-299
-673
-265
-142
-512
-573
-508
-551
-471
-270
-328
-648
-625
-779
-232
-393
-749
-84
-240
-59
-220
-55
-224
-350
-130
-23
-239
-105
-2
-762
-702
-163
-94
-350
-11
-176
-43
-654
-136
-348
-215
-67
-599
-757
-636
-367
-61
-209
-623
-342
-111
-93
-14
-613
-362
-837
-734
-468
-803
-548
-699
-744
-429
-243
-633
-382
-780
-668
-498
-664
-539
-781
-525
-697
-715
-126
-276
-504
-175
-592
-688
-92
-548
-298
-33
-532
-674
-57
-531
-488
-310
-90
-757
-496
-132
-733
-701
-61
-797
-215
-319
-700
-295
-572
-41
-140
-176
-479
-560
-164
-338
-794
-132
-453
-709
-445
-802
2
-336
-562
-802
-878
-547
-368
-502
-574
-275
-687
-560
-432
-423
-174
-367
-59
-605
-340
-626
-142
-601
-488
-299
-466
-521
-783
-140
-731
-779
-252
-663
-906
-410
-601
-524
-332
-750
-556
-730
-749
-294
-798
-93
-345
-316
-186
-634
-904
-237
-134
-765
-953
-170
-854
-910
-99
-782
-564
-505
-49
-827
-64
-297
-548
-841
-598
-414
-184
-67
-99
-880
-855
-722
-725
-582
-416
-473
-339
-491
-162
-311
-43
-938
-608
-524
-212
-4
-945
-544
-879
-382
-21
-512
-169
-284
-140
-588
-407
-56
-610
-75
-412
-321
-801
-881
-220
-388
-116
-962
-1007
-862
-20
-409
-116
-943
-558
-1001
-548
-302
-165
-730
-1012
-669
-875
-393
-979";
    println!("    * Day 5 - Part 1 ==> {}", day_5_1_2017(input));

    println!("    * Day 5 - Part 2 ==> {}", day_5_2_2017(input));

    let input = "0	5	10	0	11	14	13	4	11	8	8	7	1	4	12	11";

    println!("    * Day 6 - Part 1 ==> {}", day_6_1_2017(input));

    println!("    * Day 6 - Part 2 ==> {}", day_6_2_2017(input));

    let input = "apcztdj (61)
ulovosc (61) -> buzjgp, iimyluk
awpvs (88)
ykbjhi (14)
gxvketg (49)
gpmvdo (78) -> jjixrr, zacrh, smylfq, fdvtn
nwpmqu (6025) -> aytrde, grokih, pjqaa, nzzved
keiakhg (50)
vkrlz (22)
myzkj (40)
zkzpbbp (46)
eaumqtc (288) -> tvmvvxn, ckcpovl
htlovrb (1662) -> atxtq, ewsqqum, mjwviex
aoxky (53873) -> orzwmml, tcvtul, ygszhn
uhfiqwc (81)
gziopi (189) -> hboommj, glbipl, evojkum
rjksjq (66)
rwyzrl (1434) -> geiethw, nclvb, riphudi
gmusjd (98)
vjmwzo (76)
xymsu (153) -> kbgmlab, yjjewyt
mgiumh (64)
cnfcuww (40)
pgeoixp (211) -> qgrtg, svtcvi, rutmxs
zwwpva (89)
otgnir (87)
vxftbc (60)
xxpfmp (32) -> cnqemd, jmujchg, yfolyk, zalahl
tcrcyis (1098) -> tyealpm, dirvqby
ffjshhj (65)
powwpec (23)
gudpriq (171) -> zqxyn, bwxgrhf, dclnnlg
nzrbmfg (7)
fgptt (210)
hpcafa (91)
urepv (2628) -> qzcqhsk, iqeynjk, fakqhi
qzcqhsk (516) -> ujjksl, qccwy, cddbi, cvpoqqk, nnhlcn
riphudi (35)
nwtvzap (101) -> iwumx, bpazmfd
xhxctp (84)
dkpmgrq (94)
rvftpif (5)
iqeap (27)
aplauwh (693) -> rjkzf, nfvns, snhww
iayrwd (81) -> beidb, lpjxpx, mtvxp, alcfp
nsuysii (86) -> aktzpym, gdckok
zbxbe (59) -> ahwtsi, uqjcaa
yhiwu (37)
qpcmzl (28)
jmujchg (48)
germn (218) -> mbdwm, kjxac
mxsvay (328) -> stuom, twgcf, mexagfy
sneog (20)
rkczq (180) -> mdfae, rrsfv, yycys
nnbfqh (819) -> guttanj, htmjx, rwlmsx
nxzafkd (60)
tptdvct (64)
mokvas (49) -> ygfcl, xxitunl
wcztr (133) -> chxurcq, hemhd, siwvncp
satie (61)
vcjpuw (43)
hzcmmh (90) -> ivbwdn, eknlt, nzfjwtq, hwcbsyu, gejaump, xymsu, ebtoke
jimigk (68)
vzfebn (196) -> elqlx, lkffdz
lcwpm (1946) -> jwjuylx, pjmwkz, geajywv, zyqfc
iimyluk (65)
zcgaxsi (49)
elhmwsx (92)
bxsxr (189) -> msowvgh, zjxlr, yylzxb
qgrtg (14)
gnupj (74)
aktzpym (72)
gvuwq (88)
iojjekn (160) -> gnhcs, homrc
gzdtco (88)
ospfv (5)
epovby (14)
jflns (39)
hmqam (161) -> tkiheq, flrtvno
koxyrpn (94)
witxvcj (44)
pseltr (70) -> bzqmgl, vlmzfaf
yrotnfh (68)
fyqlqkg (19) -> gicwtf, dlxubg, ysrwm, qjnela, djupte, rayzwrp, mgexekz
eipyu (84)
hacgqe (54)
nssvbj (5)
maugts (7) -> owizuw, dopwkoo, nnbfqh, rwyzrl, kkfio, gzpcl
uiclmg (14)
psyem (173) -> tbxxc, bodpwem, lfhmte, pyobp, znnjzzw, pcnbkxm, tolty
rskopr (68)
mhihjb (68)
bfkpjdl (150) -> mpbui, qcqpavg, lwmblwr
ezotk (50)
dyadgtm (7) -> zcvzrja, rifmmf
npipd (20)
ozqsvif (231) -> stketdd, jpedzjk, vrpoaq, hbollnl
lxshx (67) -> peocao, vqsxe, tstuc, pqwzgz, ycuajn
kdersld (98) -> nrarwj, ajyyjfx
ontcbns (111) -> jptxyhp, zwwpva, ndoecxy
vyzksry (99)
sxygbpo (27)
owayyaw (5346) -> ykvqh, ruejmc, oqrvpt, qmvebxb, macxvi
paqzzv (60) -> nujgng, krnjk
qhutc (168) -> ykbjhi, uiclmg
ofndvr (46)
vfgomle (56)
znkru (32)
eymsjfd (31)
dvetl (124) -> kdccrl, rtsji
pvdbhh (96)
ggilocf (55)
wjjmh (36)
zqxyn (14)
qweyxa (32)
tiirxay (9)
avsse (292) -> wchlqxi, griwu
fmtox (6) -> mrflo, ikpyx, tsfrlzw
qczgclr (90)
tajeklj (151) -> kmrbuvq, hiaoc
kxtzbj (49)
dopwkoo (990) -> hnrwa, nwtvzap, ruitlxx
gyqrut (1431) -> pykcza, kxxhn, tipdlf
eknlt (177) -> erphv, dhahfgv
pnpge (49)
teeunu (4569) -> hirvz, mkpgx, dnflzcz
kkfio (207) -> iayrwd, abofflv, cpsmrch, ndtcui
wkdxdem (78)
dnslk (50)
civkkh (38)
bzqmgl (91)
gxtnep (43)
hemhd (54)
nzzved (110) -> vexun, sekcqad
tnejqkf (47)
jenxqco (14)
tlsdvhf (79)
kcltvch (65)
tqyheu (18)
csoko (1055) -> hnwpfl, rstsr, icnwzox
jmgsk (54)
rayzwrp (244) -> pydrzyk, hbomk
dmqhi (31)
wsctytx (26)
kjxac (65)
vrryj (177) -> nssvbj, gzlfqg
vmvdhds (33)
iuuve (90)
wbugvc (938) -> hpmynx, cznyy, yjqbsxx, yzcddkw
hnrwa (78) -> fbzro, iempnd, ojrsv
inojzm (100) -> zxlfwi, homdmyc
cvvex (75)
thumfdk (365) -> hcvvfm, oyuklmp, vtnddpz, egzpkj
wkqnq (8)
ulcnxii (59)
kdccrl (96)
hpmynx (112) -> kdmprsg, jvdawi
iempnd (35)
rqzkom (305) -> bvdqhfs, ubiuve, vqlqq
stuom (189) -> tffhcft, lzwplw
bjqzgqj (75)
ygfcl (69)
qakcvf (117) -> udvdho, gxtnep
vxlonb (88)
tjlsmrz (18) -> pmznzoa, dtifm
golth (22)
xdmqa (18)
xsilcvz (92)
oudxhh (81)
homdmyc (25)
zyrztc (15)
mmngri (60)
vqsxe (220) -> prxex, jhggy, zkmdfh
lyeoeff (74)
mcnek (54)
ckcpovl (55)
vqmeer (359) -> ijbth, phoic
zhgql (35)
zalahl (48)
toccxvy (58) -> ryecm, embmhv, wutkvkd, qlxioka
sgvbpct (13)
ubaaa (2460) -> ivkvhf, tjzut
yxmycw (1258) -> mokvas, lzcant, vrryj
mxtqhi (286) -> jwpbg, kizufjn
ofqav (13)
gqpqzrh (49)
jgjbcux (23)
vojblx (143) -> ytcgcq, bltsiq
qrbiq (7)
qnofu (64)
tcvtul (372) -> vxlonb, xcvdoi
spcpfe (255) -> tseydw, cfdumg
ohyhw (153) -> xttop, hnocmo
dqpcn (310) -> witxvcj, qshxib
btxts (816) -> klgsp, thumfdk, kbothlh
ezptzx (86)
dtifm (68)
gicwtf (254) -> beocjuy, kgfums
wvawj (59)
cddbi (255) -> ekrym, cysyc
rimlwoh (64)
wutkvkd (25)
hqlni (74)
vwzein (18)
nxrlaw (74)
nhrla (34394) -> idfyy, nwpmqu, owayyaw
whotly (17)
prxex (5)
mkpgx (468) -> thunt, lujir, xjoid
qmklslb (19)
lokvk (67)
xecai (12)
mucnt (77)
daaldt (68)
rtatr (142) -> xkjzrg, btufku
hpouwfs (22)
nclvb (35)
dnirbtx (49)
pykcza (52) -> gkbpz, vzbywuf
djplc (18)
hflkio (88) -> elhmwsx, iovcix, twffuag, sygcnhv
nkdtsf (33)
cvzvo (139) -> gfrwxw, gjphrw
ilgee (25)
zyajdwf (273) -> eymsjfd, dmqhi
rwlmsx (212) -> djsbpg, qrbiq, fnsceun, tdymtd
uhwtkvo (66)
hzzlebr (20)
mundgs (93)
ytjaofr (149) -> hmfvzi, bzplleg
epbevf (26)
ptydckb (47)
hjuzxv (74)
eljbay (138) -> rntsalv, llpxo
kvnom (87)
qvdrme (205) -> ffjshhj, kcltvch
zbwrrsc (68)
jmifnxw (42)
pfojn (42)
wgdrxl (85) -> notot, jogicnh
wxzenj (224) -> swojvy, sidmmmy
iekjt (91)
quhxdb (28)
pydrzyk (50)
hboommj (21)
vbiyre (65) -> kdrmsgh, lngffo, dqyifwc
sqehayt (1007) -> vyzpzww, hmqam, qakcvf, dyadgtm
keuqd (30) -> zrypv, zogeax, yyjfus
tyjzoq (24) -> utvyxj, pwuhv, ombqq, inattlb
xbfmp (37)
aytrde (56) -> hfvkmf, vyzksry
oufikpd (23)
qpfybmv (189) -> znkru, cjlvlm
hgsdq (288) -> bvxiqzm, jmifnxw, pfojn, bltkr
nopjkom (1447) -> ayxuibs, krcntq, hbbhm, rehezo
wtnmryv (92)
mexagfy (235) -> leoti, uqwbtx
bmsje (87)
jkoyzxm (32)
egzpkj (12)
hirvz (774) -> cmhoj, eljbay, inojzm
mxdluii (90)
unuwu (41)
ekuohb (24)
xzqgkqn (87) -> mbqdugj, lujurf
rzehh (98)
dqqyiwm (61)
oqialc (192) -> uhptzfg, quhxdb
jwjuylx (63)
ebwifj (14)
grkauy (39)
hvizfxx (83)
pyxypca (64)
gdckok (72)
yccfitx (69)
eisyowo (83) -> jjzbzsb, vbiyre, oqialc, uzxjxjx, tgjdv, ydhoryg, kdersld
mjwviex (79) -> gnupj, lazcc
grokih (202) -> wsctytx, qvksmrd
vpfoaw (36)
wtxndn (94)
lazcc (74)
pxzzw (55) -> dhccilz, rvzppdg, qcwhzgo, htlovrb
jjzbzsb (88) -> cnfcuww, gtitbbk, myzkj, rfsrdn
hrjil (71) -> ftmftoa, vdekve, lizqvr, rtkydv
ddblq (28)
eedxiav (257) -> jdeqa, tbmflh, zpjxsi, kgamsy
ywxionz (239) -> gdkll, xecai
ateol (34)
hlcahpm (90)
rntsalv (6)
zcvzrja (98)
kbbyj (87)
btufku (80)
hygdtn (7)
yxbwlf (98)
ivkvhf (9)
mevvf (60)
jdeqa (28)
jhdmh (956) -> qkilfj, xzzar, gkqrml
qlpft (57)
delev (30)
ndoecxy (89)
lfhmte (334) -> qmklslb, ekcuavb
fakqhi (1370) -> kqhggwy, zvpaj, zbxbe
mzvjxgp (41)
xuqhpz (49)
rkfknhv (28)
fbzro (35)
vrmevdy (35)
mysslj (49)
aobgmc (341) -> tajeklj, rkczq, ohyhw, zceovs, gudpriq
cocplr (67) -> opczxnj, ulcnxii, hjqvntn, wvawj
fnsceun (7)
yjqbsxx (122) -> ybtkkgv, fgyjop
atxtq (181) -> oufikpd, czacja
qvksmrd (26)
gqmjvky (137) -> xvhnaf, cwspi
bpusqb (84)
ftmftoa (101) -> nlixy, mckkn, rpbawy
aqshqpo (56)
xsnmq (46)
bltsiq (80)
towwl (54)
anhldnx (2024) -> kvnom, jbprgm
ksioqg (8)
nnunune (227) -> jmgsk, mefyd
jvdawi (56)
icnwzox (236)
qiikh (127) -> zmveluw, ycncnr
pjmwkz (63)
pvqid (50)
fivcv (64) -> pyydxvi, tzoefdo
zacrh (44)
smylfq (44)
qcwhzgo (2040) -> aetlth, kqbrwt, ualcokx
uwauv (196) -> daojy, sfyqi
mjfbr (910) -> ruuovw, kghzy, psyem
efcff (79)
kzflj (19)
shethb (91) -> qsgpsfr, fmtox, pseltr, gziopi, ohqlwmr
yylzxb (6)
ybtkkgv (51)
olxmnkc (79) -> wtnmryv, encean
cysyc (8)
tkiheq (21)
xkjzrg (80)
vtnddpz (12)
kgfums (45)
rrlwo (41)
rujggz (68)
beidb (63)
xcvdoi (88)
xghng (17)
vmoqov (43)
pekzuyh (29)
fheklq (66) -> crdlymr, vgsuq, lqspkzw, wbugvc
wodhzc (18)
rlzbv (92)
ycuajn (185) -> iizggi, ilgee
zjrpxp (73)
sogepf (66)
gbecv (78) -> hlcahpm, mxdluii, qczgclr
xzvbcms (92)
wcklpgi (14)
wuhossn (96)
dlrfwee (35)
rfsrdn (40)
akkrsv (108) -> ontcbns, mrydp, flqjd
uqjcaa (54)
ixcxv (49) -> ruvrw, rwqjiz, towwl
kdrmsgh (61)
gtxlyzz (32)
udvdho (43)
lelvcai (28)
ivbwdn (121) -> hpocx, bwpqytq
bvdqhfs (6)
wrxzti (62)
yoofo (79)
oqrvpt (323) -> pyjzb, wkqnq
bfxdtkg (93)
yrmmga (36)
tvmvvxn (55)
hlhpm (187) -> xqnan, ykubvpo, bxsxr
ovalfky (66)
wpner (54)
gljri (96)
jpedzjk (153) -> byphua, nxzafkd, ccpek
ydhoryg (65) -> apcztdj, wfuxy, satie
zxsgnx (105) -> pnumlg, gouyeqw
zyqfc (63)
evxjdeo (57)
uspfk (54)
rwqjiz (54)
griwu (72)
mcgaqx (68)
iqzkp (103) -> rkfknhv, lelvcai
edmsh (77)
uavzab (17)
seleq (87)
wsozlg (7)
dgvvi (93)
svbmkbx (196) -> golth, casbq
szoomk (26)
bcima (78)
cyasn (4978) -> hlhpm, fmmhxab, chirgx
hfvkmf (99)
fjonxe (73) -> xbfmp, yhiwu, kxchm
wnoer (81)
zaouq (96)
gkqrml (109) -> umitnao, bkcgx
ppythr (73)
qozshd (14)
namzem (66)
cjlvlm (32)
ohcfrxm (85) -> xhxctp, wxpmbu
iqwspxd (897) -> gqmjvky, wyzmc, mbzme
chirgx (46) -> gpmvdo, fivcv, aelsfhi
pyobp (78) -> gmusjd, pqfnj, xbhcwe
kspoky (9)
gtkqi (56) -> cquptt, cdeji
twgcf (49) -> lokvk, cpxwa, yovtgg, nzzwed
plsic (60)
pmyqol (66) -> ayixid, efcff
hnmba (22)
ktbtfdb (56)
yyjfus (85)
rvzppdg (45) -> airnuh, mfvqvf, begmhqg, cykvv, wipzuag, fmstmh
vebyb (30)
yasoqri (41)
gjphrw (85)
caqjohx (25)
oqiynm (9)
lydynp (73)
dclnnlg (14)
nocugh (81)
tqzbg (15)
elqlx (81)
mckkn (67)
lujir (152) -> pvqid, keiakhg
mywhdsm (47)
oafxzu (27)
fgumszu (98)
hnwpfl (89) -> gqpqzrh, nlrwlj, gxvketg
xrvno (49)
kjdjj (225) -> keuqd, yompk, grxqt, ocxbgp
vyzpzww (163) -> npipd, hzzlebr
oqzcbx (81) -> wuhossn, ebhag, zaouq
twffuag (92)
nujgng (82)
xbhcwe (98)
bvugv (199) -> xdmqa, qqszul, wodhzc
vgsuq (1510) -> oanoax, swcxes, qmelelp
cwflw (27)
yraktfn (27236) -> pxzzw, igwew, jjzlrku
iqeynjk (17) -> dccvh, cvzvo, kcjujcr, phqplx, spcpfe, dygrnnf
vgowu (30) -> mgiumh, pyxypca
jjzlrku (635) -> fuwbaq, lcwpm, oahpf, anhldnx
zkmdfh (5)
dvskig (22)
zaitni (83) -> iqagnh, ixckwlm, kfvvzd
ikthp (51)
jvpob (76)
cpxwa (67)
sphwh (88)
mlbjc (81) -> fylao, ggilocf
rwmci (63) -> qnofu, rimlwoh
fluuo (51)
exvmk (78)
jgdqi (3125) -> mxsvay, hrjil, fhtktd, qqvgeq
aplal (124) -> ezotk, mithbv
gouyeqw (27)
iqagnh (47)
xfjafp (78)
gvaie (153) -> cpwdhw, bjqzgqj
ijbth (5)
zduooao (93)
tjlrb (78)
emeyn (35)
zblcbd (34)
xxtki (88)
nxgjqaf (1807) -> ubaaa, hjxhx, uyofsmv
xqnan (87) -> ckbuzb, rutqvq
xyquxix (70)
pwuhv (72)
sygcnhv (92)
qjlfwtm (19)
vkkgwbr (8)
casbq (22)
zeisn (95)
rplqlc (63)
ewsqqum (99) -> mojwgp, zuxgf
mojwgp (64)
dexul (230)
kdmprsg (56)
amifh (43)
notot (63)
byphua (60)
vfjayw (88)
gfrwxw (85)
vrpoaq (41) -> mwzpa, pfhjtw, rthxl, ppythr
wngqx (26)
obgmlzf (67)
daojy (17)
vxnwq (61) -> kjhrii, ulovosc, ytjaofr, vrgblbn, rwmci, mlbjc, vblppqb
yzcddkw (40) -> hvnzfbd, hqnonqc
iiswvjq (62)
yeerc (68)
ycncnr (42)
cmrcpf (294) -> onbptkd, qxsxzth
zbtlfr (35)
paqggr (84) -> ettsdu, iekjt
prfbji (14)
bodpwem (244) -> jkoyzxm, gtxlyzz, qweyxa, bbqzlo
ccpek (60)
buzjgp (65)
ahwtsi (54)
ckbuzb (60)
mrflo (82)
alcfp (63)
engay (35)
fxeka (18)
ohqlwmr (112) -> tdcxeia, maaid, fiygpep, zhgql
iizggi (25)
qcqpavg (364)
siwvncp (54)
lpjxpx (63)
ryecm (25)
xrsaryj (115) -> qcrxve, iuuve
dterxr (244) -> hfxmz, mpble
kcjujcr (207) -> eyavk, ikthp
sjommm (98)
lneqx (456)
dccvh (201) -> uipvt, iqeap, gbuilbn, oafxzu
duzorty (20)
mdfae (11)
pmavka (48)
ekrym (8)
logpgb (2713) -> wamxgvn, ozqsvif, olnpj
pxrxqs (78)
thunt (94) -> tlsdvhf, yoofo
ogkucp (84)
xroaal (43)
czacja (23)
xbaytg (5)
embmhv (25)
ahsef (41)
cfbjnxb (799) -> ujpka, fjonxe, slacopj
itusrhj (277) -> gscng, pekzuyh
vhruv (34)
fdvtn (44)
flrtvno (21)
fiygpep (35)
fhtktd (1103) -> xxtki, pgnulg
dirvqby (72)
codroxx (49)
fswirnb (98)
rvzqqr (290) -> wafsr, pwhrvu
peocao (235)
drleu (70)
ygszhn (86) -> lnuqdd, tjlsmrz, yomawy
ixbxszv (43)
nzfjwtq (67) -> douggpq, ezptzx
wfmxgw (98)
gzuqs (80)
ujpka (84) -> bhpvfr, dnslk
ebjpdvd (19)
mykkoc (91)
pyydxvi (95)
prybce (62)
smpmp (9)
nfvns (183)
rvqubk (81)
ezvdo (74)
gdkll (12)
tzoefdo (95)
vzdmsv (224)
vcusbhv (196) -> ospfv, wfvtfos, yjdsnqu
beocjuy (45)
zjxlr (6)
dlxubg (344)
kfvvzd (47)
zrypv (85)
aonpk (25)
icsrp (14)
pjthjam (286) -> ofqav, sgvbpct
fgyjop (51)
rstsr (68) -> xnleu, gfvjqj
xgrjpl (92)
rlzobr (89) -> ctifv, ffevqd, kbbyj, otgnir
vbvctl (87)
lqspkzw (90) -> rvzqqr, avsse, rocllr, cpiscj
mbqtji (392) -> paqjozd, wsbpne, tncexxs
vuumtry (63)
zpjxsi (28)
cykvv (247) -> yeerc, qfyrdo
chgpk (281) -> sxygbpo, hkyqakf
wchlqxi (72)
lzwplw (64)
daonbjx (790) -> ohcfrxm, yinhnv, bvugv, pgeoixp, qpfybmv
ffevqd (87)
gnhcs (78)
yjdsnqu (5)
vjugmg (73) -> qekol, jjdcuql, atdrk, rqzkom
swcxes (60) -> bzwhd, ekuohb
hmfvzi (21)
wwcrqwq (54)
iezlg (71) -> maugts, nxgjqaf, liibv, qrvmp, nwyqp, mjfbr
erphv (31)
vpxbz (48) -> qaalu, gtutr
sbsustu (91)
uvwnhot (14)
mbdwm (65)
cwspi (15)
fnvxe (104) -> bwmckc, vxftbc
qmxjyg (56)
hkyqakf (27)
uyofsmv (1214) -> ukdbm, iojjekn, dterxr, dvetl
glbipl (21)
yjjrrjj (93)
xnleu (84)
dyeffd (64)
mpbui (52) -> bcima, yowna, hvpzbd, smzzu
hvnzfbd (92)
vguwn (40)
tdcxeia (35)
dhccilz (471) -> wxzenj, tyjzoq, azrdj, mxtqhi, pjthjam, zzjicx
mfvqvf (355) -> rcdic, ebwifj
qsjue (81)
kgnbx (56)
efsdxm (74)
niojmd (95)
uzxjxjx (68) -> plsic, mmngri, mevvf
drbjnk (99)
hqnonqc (92)
opczxnj (59)
iwumx (41)
jwpbg (13)
aetlth (7) -> tnejqkf, mywhdsm
ujjksl (169) -> ixhsq, fluuo
ygqjcm (74) -> uqpxyi, uxmfcyr, oudxhh, iktols
oyjjdj (678) -> svbmkbx, lxpkrf, vayyag
pnumlg (27)
hbyxgv (94)
ivhzxr (14)
geajywv (63)
kjhrii (135) -> ddblq, ehkhs
rehezo (56) -> vuumtry, rplqlc, zcakvl
gunfz (80)
airnuh (369) -> hygdtn, nzrbmfg
ruejmc (283) -> icsrp, ivhzxr, uvwnhot, jakny
ikpyx (82)
zxlfwi (25)
ocxbgp (285)
qfsbvqe (39)
htmjx (48) -> tptdvct, smhzr, dyeffd
ukdbm (44) -> rskopr, rujggz, mhihjb, jimigk
lqtuij (66)
jogicnh (63)
vqlqq (6)
bkcgx (80)
fxsmo (85)
icoiqa (256) -> ljscsoa, sxqnium
ehhrom (334) -> aqshqpo, kgnbx
tgrfyrx (6071) -> urepv, ippmv, yvirt, mofdlro, teeunu, jgdqi
kttzsv (49)
hpmag (88)
jjixrr (44)
rjkzf (147) -> fxeka, pealz
rtsji (96)
wfuxy (61)
dzjef (22)
gfvjqj (84)
lslsl (28)
lizqvr (101) -> zaeyze, ppbgib, obgmlzf
rcdic (14)
ykiwb (105) -> drbjnk, vwokfc
dygrnnf (153) -> pxrxqs, xfjafp
csbdh (28)
tffhcft (64)
eilgcny (62) -> ldfybwl, gzuqs, gunfz
hfxmz (36)
spqwtg (1089) -> foaubeo, ehhrom, fcuhw
gtitbbk (40)
wsbpne (15)
gpcxuo (81)
cpsmrch (333)
fhmkuo (266) -> zeisn, niojmd
ckifyf (23)
ppbgib (67)
titychs (25909) -> fqmyi, logpgb, cyasn, fheklq
hlyfdr (26)
zmveluw (42)
kqhggwy (167)
cakob (96)
wipzuag (187) -> rerviwo, sjommm
hvpzbd (78)
vexun (72)
fuwbaq (13) -> ytzrx, pwyiwkf, rlzobr, mbqtji, jpnfrsb
fqmyi (3349) -> wxirji, cfbjnxb, shethb
xbgmzxd (144) -> wsozlg, lzqtms
paqjozd (15)
jakny (14)
susknu (81)
olnpj (87) -> rhsdbfh, vqmeer, oqzcbx, eedxiav
hjrzrli (62)
tjzut (9)
kizufjn (13)
iduxx (9)
wfvtfos (5)
dhahfgv (31)
oahpf (630) -> pmyqol, zaitni, xxpfmp, aplal, vzdmsv, paqzzv, fnvxe
utvyxj (72)
andixsk (39)
nglumw (49)
cdeji (59)
ynxfzsu (73) -> amifh, xroaal
hpljf (37)
oanoax (52) -> csbdh, tfdjhaa
jbxcafx (97) -> qlpft, evxjdeo
xaplszi (689) -> qgbtnil, icoiqa, vzfebn
sbtyi (91)
wnhmooe (37)
qekol (35) -> gljri, cakob, pvdbhh
lkffdz (81)
hjxhx (42) -> germn, ycrxr, cmrcpf, tbrxke, gbecv, wnakg, epdqjdp
ualcokx (35) -> nkdtsf, vmvdhds
hwofq (37) -> izjbi, dqqyiwm
bzmra (34)
gnuywz (30)
rutqvq (60)
kvgrf (38)
rpbawy (67)
uunjwu (70)
nlrwlj (49)
ettsdu (91)
pyjzb (8)
ehhlk (75)
vdekve (292) -> xbaytg, rvftpif
ycrxr (312) -> twvlr, djplc
pdewbw (74)
xzzar (239) -> tqzbg, zyrztc
hwcbsyu (167) -> wjjmh, uwwyv
msowvgh (6)
jptxyhp (89)
ulkzj (39)
ytzrx (275) -> uspfk, wpner, wwcrqwq
lunivx (78)
tfdjhaa (28)
lzqtms (7)
qsgpsfr (96) -> utneufs, exvmk
qmvebxb (43) -> hjuzxv, nxrlaw, pdewbw, hqlni
qkilfj (99) -> mohya, fxsmo
twvlr (18)
fejmhpg (190) -> kvgrf, civkkh
lzcant (149) -> gnoywf, ebjpdvd
fcuhw (390) -> qpcmzl, ujdil
qlxioka (25)
tyealpm (72)
ndbzyu (68)
djqtfp (68)
wlbua (48)
xuhchjm (80)
mwzpa (73)
dqyifwc (61)
tovvj (8)
rthxl (73)
bhpvfr (50)
geiethw (35)
yinhnv (22) -> mucnt, edmsh, apgzt
abofflv (105) -> vjmwzo, jvpob, ddpmrsd
ayxuibs (89) -> kgxseg, tjlrb
ebcygk (42) -> cehqdz, qmxjyg, pwpmx
ebhag (96)
pqwzgz (189) -> jgjbcux, powwpec
pmznzoa (68)
fylao (55)
bbqzlo (32)
chxurcq (54)
egdccd (54)
yghyn (72)
zuxgf (64)
apgzt (77)
ujdil (28)
lwmblwr (208) -> ingwpue, jfwho
oyvaqjr (80)
maaid (35)
pcnbkxm (93) -> bfxdtkg, zduooao, mundgs
gscng (29)
tolty (336) -> vwzein, tqyheu
nzzwed (67)
ruhyv (29)
wxpmbu (84)
izjbi (61)
jbprgm (87)
jpnfrsb (290) -> codroxx, xrvno, pnpge
hcvvfm (12)
yowna (78)
mithbv (50)
qcxfwey (81)
pgnulg (88)
bxucoyp (70)
sxqnium (51)
bzwhd (24)
tbrxke (250) -> kxtzbj, zfuqmcg
hnocmo (30)
hbollnl (225) -> qrbvsw, mcnek
pfhjtw (73)
mrydp (324) -> vswcvmu, cwflw
envbl (113) -> ehhlk, cvvex
swojvy (44)
gzlfqg (5)
vblppqb (164) -> qhwkoc, oqiynm, kspoky
rocllr (88) -> tvixzc, seleq, bmsje, vbvctl
vlmzfaf (91)
jfwho (78)
llpxo (6)
pwhrvu (73)
kxxhn (126) -> ahsef, rrlwo
evojkum (21)
cpiscj (436)
qhwkoc (9)
grxqt (233) -> mtzvap, wngqx
rhsdbfh (185) -> rlzbv, xgrjpl
mbqdugj (62)
aelsfhi (58) -> rzehh, fswirnb
douggpq (86)
ysrwm (206) -> zkzpbbp, xsnmq, ofndvr
fjtgkp (78)
tsfrlzw (82)
kghzy (1262) -> gvaie, ykiwb, wztfz, vojblx, cocplr
gzpcl (1291) -> qgzaj, wrxzti, hjrzrli, qklbf
ykryfdg (1297) -> aoqukac, gtkqi, cjhcn
kqitg (203) -> gnuywz, wwolc
bwmckc (60)
yovtgg (67)
irducr (56)
encean (92)
tseydw (27)
tyhtbv (28)
macxvi (179) -> oyvaqjr, arqvl
qklbf (62)
hpocx (59)
qccwy (85) -> yjjrrjj, dgvvi
mehtsc (84)
djupte (20) -> wnoer, qcxfwey, rvqubk, gpcxuo
meuyumr (810) -> vpxbz, mhpbfn, qhutc
crdlymr (1705) -> ixbxszv, vcjpuw, vmoqov
suhcj (14)
lujurf (62)
pjqaa (118) -> bzmra, ateol, zblcbd, qwtlvuj
ddpmrsd (76)
foaubeo (70) -> hbyxgv, totzxaf, dkpmgrq, wtxndn
ykubvpo (133) -> hpljf, wnhmooe
ljscsoa (51)
wztfz (269) -> whotly, evpryza
qaalu (74)
zzjicx (146) -> hvizfxx, xlukbym
ykvqh (305) -> uavzab, xghng
tbmflh (28)
bpazmfd (41)
bwxgrhf (14)
vswcvmu (27)
nnhlcn (201) -> emeyn, vzyxst
vzyxst (35)
qrvmp (426) -> xaplszi, jhdmh, hzcmmh, kkhvw, csoko
kmrbuvq (31)
uqpxyi (81)
tstuc (59) -> sphwh, hpmag
kbothlh (149) -> uhwtkvo, ovalfky, lqtuij, sogepf
xvhnaf (15)
qzsjiw (40)
krnjk (82)
rifmmf (98)
cnqemd (48)
ombqq (72)
xmvxrop (48) -> eaumqtc, dqpcn, ygqjcm
pwpmx (56)
jhggy (5)
ruvrw (54)
qcrxve (90)
ldfybwl (80)
eyavk (51)
smhzr (64)
mtvxp (63)
gmcebvw (26)
hpbbqmn (84)
wafsr (73)
homrc (78)
znnjzzw (8) -> sbsustu, sbtyi, clzgzgc, mykkoc
umitnao (80)
tncexxs (15)
qwbkv (92)
tipdlf (61) -> dnirbtx, kttzsv, mysslj
wnakg (166) -> hpcafa, emecx
fxzvbzr (51) -> qqbdtow, xuhchjm
tgjdv (80) -> ogkucp, bpusqb
hiaoc (31)
onbptkd (27)
yhovo (78)
zogeax (85)
qqbdtow (80)
leoti (41)
yomawy (154)
qfyrdo (68)
rerviwo (98)
liibv (8767) -> toccxvy, xbgmzxd, vgowu
uxmfcyr (81)
cfdumg (27)
emecx (91)
hbbhm (137) -> hacgqe, egdccd
zvrld (46) -> xsilcvz, evxjdo
ixckwlm (47)
ruitlxx (67) -> jbiaxln, ekfwsv
wwolc (30)
ixhsq (51)
rutmxs (14)
sekcqad (72)
haeee (400) -> qozshd, prfbji, jenxqco, wcklpgi
uqwbtx (41)
phoic (5)
kqbrwt (43) -> ruhyv, xxpauyc
btzbl (78)
ziock (49)
yycys (11)
tvixzc (87)
frucbsg (69)
ruuovw (41) -> hflkio, lneqx, fhmkuo, haeee, jvefqe, hgsdq
wamxgvn (511) -> ywxionz, olxmnkc, kqitg, envbl
qrbvsw (54)
evpryza (17)
zjxyreo (34) -> gvuwq, awpvs
sfyqi (17)
gtutr (74)
ctifv (87)
yjjewyt (43)
mofdlro (789) -> aplauwh, tcrcyis, bfkpjdl, akkrsv, lxshx, xmvxrop
hjqvntn (59)
wxirji (553) -> pmvdtmm, fejmhpg, paqggr
qgzaj (62)
qqszul (18)
kgxseg (78)
iktols (81)
orzwmml (478) -> engay, zbtlfr
gkbpz (78)
inattlb (72)
jjdcuql (183) -> xyquxix, uunjwu
kkhvw (1133) -> zjxyreo, fgptt, ebcygk
rrsfv (11)
atdrk (285) -> kzflj, qjlfwtm
nrarwj (75)
tdymtd (7)
bvxiqzm (42)
ehkhs (28)
cvpoqqk (211) -> delev, vebyb
owizuw (62) -> ixcxv, qiikh, wgdrxl, xzqgkqn, vcusbhv, fxzvbzr, jbxcafx
kgamsy (28)
cmhoj (12) -> frucbsg, yccfitx
kbgmlab (43)
epdqjdp (244) -> epbevf, hlyfdr, szoomk, gmcebvw
vrgblbn (191)
ndtcui (239) -> umymwd, ptydckb
xjoid (154) -> xuqhpz, nglumw
mefyd (54)
cpwdhw (75)
jwasx (246) -> tyhtbv, lslsl
pwyiwkf (281) -> wkdxdem, yhovo
xcndr (8)
yvirt (960) -> nopjkom, spqwtg, fyqlqkg
xttop (30)
nwyqp (1965) -> yxmycw, sqehayt, eisyowo, ykryfdg
clzgzgc (91)
agsrs (185) -> qsnjuqh, wcztr, xrsaryj, ifqsqyz
ojrsv (35)
gnoywf (19)
wyzmc (97) -> dlrfwee, vrmevdy
tesql (72)
ejsitta (83) -> mehtsc, hpbbqmn, eipyu
ekfwsv (58)
xxitunl (69)
jvefqe (339) -> ulkzj, jflns, grkauy
vwokfc (99)
mtzvap (26)
vzbywuf (78)
cquptt (59)
fmstmh (383)
qgbtnil (86) -> daaldt, ndbzyu, yrotnfh, djqtfp
qshxib (44)
nclav (91) -> vhruv, vjnejlj
azrdj (176) -> zbwrrsc, mcgaqx
cehqdz (56)
zfuqmcg (49)
tkhjz (73)
pealz (18)
hbomk (50)
oyuklmp (12)
klgsp (89) -> uhfiqwc, susknu, nocugh, qsjue
uwwyv (36)
mhpbfn (196)
krcntq (165) -> duzorty, mcvuku, sneog, bnetz
dkvqe (222) -> qzsjiw, vguwn
ingwpue (78)
smzzu (78)
bzplleg (21)
uhptzfg (28)
qwtlvuj (34)
arqvl (80)
bnetz (20)
yfolyk (48)
flqjd (84) -> yxbwlf, fgumszu, wfmxgw
igwew (5332) -> kjdjj, agsrs, vjugmg
vjnejlj (34)
svtcvi (14)
lnuqdd (154)
evxjdo (92)
vvtfwwb (74)
snhww (39) -> yghyn, tesql
cjhcn (124) -> aonpk, caqjohx
iovcix (92)
mgexekz (212) -> namzem, rjksjq
ajyyjfx (75)
bwpqytq (59)
nvmqchp (23)
asosowy (56)
guttanj (208) -> tovvj, vkkgwbr, xcndr, ksioqg
utneufs (78)
zcakvl (63)
begmhqg (87) -> lyeoeff, efsdxm, vvtfwwb, ezvdo
pmvdtmm (220) -> ckifyf, nvmqchp
fmmhxab (13) -> zxsgnx, iqzkp, ynxfzsu, hwofq, nclav
zaeyze (67)
djsbpg (7)
rqwgj (15) -> nhrla, titychs, tgrfyrx, aoxky, iezlg, yraktfn
lxpkrf (196) -> vkrlz, dvskig
nlixy (67)
gbuilbn (27)
vayyag (56) -> qwbkv, xzvbcms
qsnjuqh (61) -> btzbl, lunivx, fjtgkp
aoqukac (50) -> iiswvjq, prybce
ekcuavb (19)
tbxxc (372)
ippmv (21) -> btxts, daonbjx, zebuqzh, gyqrut
zvpaj (69) -> zcgaxsi, ziock
bltkr (42)
dnflzcz (16) -> dkvqe, eilgcny, rtatr, jwasx
jbiaxln (58)
qmelelp (80) -> epovby, suhcj
mpble (36)
ebtoke (143) -> pmavka, wlbua
pqfnj (98)
kxchm (37)
cznyy (224)
ytcgcq (80)
phqplx (85) -> asosowy, irducr, ktbtfdb, vfgomle
umymwd (47)
zceovs (25) -> koxyrpn, emhnlu
rtkydv (83) -> zjrpxp, lydynp, tkhjz
ayixid (79)
xlukbym (83)
stketdd (157) -> gzdtco, vfjayw
yompk (162) -> unuwu, yasoqri, mzvjxgp
slacopj (44) -> bxucoyp, drleu
emhnlu (94)
mbzme (101) -> hpouwfs, hnmba, dzjef
mohya (85)
totzxaf (94)
ifqsqyz (223) -> vpfoaw, yrmmga
lngffo (61)
xxpauyc (29)
qxsxzth (27)
zebuqzh (45) -> ejsitta, chgpk, zyajdwf, itusrhj, nnunune, qvdrme
ubiuve (6)
mcvuku (20)
uipvt (27)
qqvgeq (359) -> zvrld, nsuysii, uwauv, dexul
gejaump (212) -> iduxx, tiirxay, smpmp
sidmmmy (44)
qjnela (266) -> andixsk, qfsbvqe
idfyy (51) -> vxnwq, meuyumr, oyjjdj, iqwspxd, aobgmc
";

    println!("    * Day 7 - Part 1 ==> {}", day_7_1_2017(input));
}

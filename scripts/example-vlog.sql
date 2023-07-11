-- MySQL dump 10.13  Distrib 5.7.28, for Linux (x86_64)
--
-- Host: localhost    Database: rust_admin
-- ------------------------------------------------------
-- Server version	5.7.28-0ubuntu0.18.04.4

/*!40101 SET @OLD_CHARACTER_SET_CLIENT=@@CHARACTER_SET_CLIENT */;
/*!40101 SET @OLD_CHARACTER_SET_RESULTS=@@CHARACTER_SET_RESULTS */;
/*!40101 SET @OLD_COLLATION_CONNECTION=@@COLLATION_CONNECTION */;
/*!40101 SET NAMES utf8 */;
/*!40103 SET @OLD_TIME_ZONE=@@TIME_ZONE */;
/*!40103 SET TIME_ZONE='+00:00' */;
/*!40014 SET @OLD_UNIQUE_CHECKS=@@UNIQUE_CHECKS, UNIQUE_CHECKS=0 */;
/*!40014 SET @OLD_FOREIGN_KEY_CHECKS=@@FOREIGN_KEY_CHECKS, FOREIGN_KEY_CHECKS=0 */;
/*!40101 SET @OLD_SQL_MODE=@@SQL_MODE, SQL_MODE='NO_AUTO_VALUE_ON_ZERO' */;
/*!40111 SET @OLD_SQL_NOTES=@@SQL_NOTES, SQL_NOTES=0 */;

--
-- Table structure for table `admin_roles`
--

DROP TABLE IF EXISTS `admin_roles`;
/*!40101 SET @saved_cs_client     = @@character_set_client */;
/*!40101 SET character_set_client = utf8 */;
CREATE TABLE `admin_roles` (
  `id` int(10) unsigned NOT NULL AUTO_INCREMENT,
  `name` varchar(20) NOT NULL DEFAULT '',
  `remark` varchar(50) NOT NULL DEFAULT '',
  `menu_ids` text,
  `seq` int(11) NOT NULL DEFAULT '0',
  PRIMARY KEY (`id`)
) ENGINE=InnoDB AUTO_INCREMENT=2 DEFAULT CHARSET=utf8;
/*!40101 SET character_set_client = @saved_cs_client */;

--
-- Dumping data for table `admin_roles`
--

LOCK TABLES `admin_roles` WRITE;
/*!40000 ALTER TABLE `admin_roles` DISABLE KEYS */;
INSERT INTO `admin_roles` VALUES (1,'System admin','System admin','1,2,3,4,5,6,7,8,9,10,11,12,13,14,15,16,17,18,19,20,21,22,23,24,33,34,35,25,26,27,28,29,30,31,32,36,37,38,39,40,41,42,43,44',0);
/*!40000 ALTER TABLE `admin_roles` ENABLE KEYS */;
UNLOCK TABLES;

--
-- Table structure for table `admins`
--

DROP TABLE IF EXISTS `admins`;
/*!40101 SET @saved_cs_client     = @@character_set_client */;
/*!40101 SET character_set_client = utf8 */;
CREATE TABLE `admins` (
  `id` int(10) unsigned NOT NULL AUTO_INCREMENT,
  `name` varchar(20) NOT NULL DEFAULT '',
  `password` char(32) NOT NULL DEFAULT '',
  `secret` char(32) NOT NULL DEFAULT '',
  `last_ip` varchar(32) NOT NULL DEFAULT '',
  `state` tinyint(3) unsigned NOT NULL DEFAULT '0',
  `login_count` int(10) unsigned NOT NULL DEFAULT '0',
  `last_login` int(10) unsigned NOT NULL DEFAULT '0',
  `role_id` int(10) unsigned NOT NULL DEFAULT '0',
  `created` int(10) unsigned NOT NULL DEFAULT '0',
  `updated` int(10) unsigned NOT NULL DEFAULT '0',
  `seq` int(11) NOT NULL DEFAULT '0',
  PRIMARY KEY (`id`),
  KEY `name` (`name`)
) ENGINE=InnoDB AUTO_INCREMENT=2 DEFAULT CHARSET=utf8;
/*!40101 SET character_set_client = @saved_cs_client */;

--
-- Dumping data for table `admins`
--

LOCK TABLES `admins` WRITE;
/*!40000 ALTER TABLE `admins` DISABLE KEYS */;
INSERT INTO `admins` VALUES (1,'admin','a3b12db078cc790812c5e70220cd46b1','BJbZEngUhaBO22MrlEQ2STGqvy1bxr5k','127.0.0.1',1,3,1580886502,1,1580470174,1580886502,0);
/*!40000 ALTER TABLE `admins` ENABLE KEYS */;
UNLOCK TABLES;

--
-- Table structure for table `ads`
--

DROP TABLE IF EXISTS `ads`;
/*!40101 SET @saved_cs_client     = @@character_set_client */;
/*!40101 SET character_set_client = utf8 */;
CREATE TABLE `ads` (
  `id` int(10) unsigned NOT NULL AUTO_INCREMENT,
  `name` varchar(20) NOT NULL DEFAULT '',
  `remark` varchar(100) NOT NULL DEFAULT '',
  `image` varchar(200) NOT NULL DEFAULT '',
  `page_id` int(10) unsigned NOT NULL DEFAULT '0',
  `position_id` int(10) unsigned NOT NULL DEFAULT '0',
  `url` varchar(200) NOT NULL DEFAULT '',
  `is_blank` tinyint(3) unsigned NOT NULL DEFAULT '1',
  `seq` int(11) NOT NULL DEFAULT '0',
  PRIMARY KEY (`id`)
) ENGINE=InnoDB AUTO_INCREMENT=2 DEFAULT CHARSET=utf8;
/*!40101 SET character_set_client = @saved_cs_client */;

--
-- Dumping data for table `ads`
--

LOCK TABLES `ads` WRITE;
/*!40000 ALTER TABLE `ads` DISABLE KEYS */;
INSERT INTO `ads` VALUES (1,'Athlete spirit','Treat the athlete to the spirit','',0,0,'',1,0);
/*!40000 ALTER TABLE `ads` ENABLE KEYS */;
UNLOCK TABLES;

--
-- Table structure for table `configs`
--

DROP TABLE IF EXISTS `configs`;
/*!40101 SET @saved_cs_client     = @@character_set_client */;
/*!40101 SET character_set_client = utf8 */;
CREATE TABLE `configs` (
  `id` int(10) unsigned NOT NULL AUTO_INCREMENT,
  `site_name` varchar(50) NOT NULL DEFAULT '',
  `site_url` varchar(200) NOT NULL DEFAULT '',
  `seo_keyword` varchar(250) NOT NULL DEFAULT '',
  `seo_desc` varchar(250) NOT NULL DEFAULT '',
  `copyright` varchar(200) NOT NULL DEFAULT '',
  PRIMARY KEY (`id`)
) ENGINE=InnoDB AUTO_INCREMENT=2 DEFAULT CHARSET=utf8;
/*!40101 SET character_set_client = @saved_cs_client */;

--
-- Dumping data for table `configs`
--

LOCK TABLES `configs` WRITE;
/*!40000 ALTER TABLE `configs` DISABLE KEYS */;
INSERT INTO `configs` VALUES (1,'Website name','http://site.cn/','Website keywords used for SEO','Website description for SEO','Website copyright information');
/*!40000 ALTER TABLE `configs` ENABLE KEYS */;
UNLOCK TABLES;

--
-- Table structure for table `menus`
--

DROP TABLE IF EXISTS `menus`;
/*!40101 SET @saved_cs_client     = @@character_set_client */;
/*!40101 SET character_set_client = utf8 */;
CREATE TABLE `menus` (
  `id` int(10) unsigned NOT NULL AUTO_INCREMENT,
  `parent_id` int(10) unsigned NOT NULL DEFAULT '0',
  `name` varchar(50) NOT NULL DEFAULT '',
  `level_id` tinyint(4) NOT NULL DEFAULT '0',
  `state` tinyint(4) NOT NULL DEFAULT '0',
  `url` varchar(200) NOT NULL DEFAULT '',
  `is_blank` tinyint(4) NOT NULL DEFAULT '0',
  `is_show` tinyint(4) NOT NULL DEFAULT '0',
  `seq` int(11) NOT NULL DEFAULT '0',
  PRIMARY KEY (`id`),
  KEY `parent_id` (`parent_id`)
) ENGINE=InnoDB AUTO_INCREMENT=48 DEFAULT CHARSET=utf8;
/*!40101 SET character_set_client = @saved_cs_client */;

--
-- Dumping data for table `menus`
--

LOCK TABLES `menus` WRITE;
/*!40000 ALTER TABLE `menus` DISABLE KEYS */;
INSERT INTO `menus` VALUES (1,0,'BE management',0,1,'#',0,1,0),(2,0,'Content management',0,1,'#',0,1,0),(3,0,'FE management',0,1,'#',0,1,0),(4,1,'Admins',1,1,'/admins',0,1,0),(5,1,'Add/edit admin',1,1,'/admins/edit/\\d+|/admins/save/\\d+',0,0,0),(6,1,'Delete admin',1,1,'/admins/delete/\\d+',0,0,0),(7,1,'Menus',1,1,'/menus',0,1,0),(8,1,'Add/edit menu',1,1,'/menus/edit/\\d+|/menus/save/\\d+',0,0,0),(9,1,'Delete menu',1,1,'/menus/delete/\\d+',0,0,0),(10,1,'Roles',1,1,'/admin_roles',0,1,0),(11,1,'Add/edit role',1,1,'/admin_roles/edit/\\d+|/admin_roles/save/\\d+',0,0,0),(12,1,'Delete role',1,1,'/admin_roles/\\d+',0,0,0),(13,2,'Video categories',1,1,'/video_categories',0,1,0),(14,2,'Add/edit video category',1,1,'/video_categories/edit/\\d+|/video_categories/save/\\d+',0,0,0),(15,2,'Delete video category',1,1,'/video_categories/delete/\\d+',0,0,0),(16,2,'Video tags',1,1,'/video_tags',0,1,0),(17,2,'Add/edit video tag',1,1,'/video_tags/edit/\\d+|/videos/save/\\d+',0,0,0),(18,2,'Delete video tag',1,1,'/video_tags/delete/\\d+',0,0,0),(19,2,'Videos',1,1,'/videos',0,1,0),(20,2,'Add/edit video',1,1,'/videos/edit/\\d+|/videos/save/\\d+',0,0,0),(21,2,'Delete video',1,1,'/videos/delete/\\d+',0,0,0),(22,2,'Video replies',1,1,'/video_replies',0,1,0),(23,2,'Add/edit video reply',1,1,'/video_replies/edit/\\d+|/video_replies/save/\\d+',0,0,0),(24,2,'Delete video reply',1,1,'/video_replies/delete/\\d+',0,0,0),(25,3,'Users',1,1,'/users',0,1,0),(26,3,'Add/edit user',1,1,'/users/edit/\\d+|/users/save/\\d+',0,0,0),(27,3,'Delete user',1,1,'/users/delete/\\d+',0,0,0),(28,3,'User levels',1,1,'/user_levels',0,1,0),(29,3,'Add/edit user level',1,1,'/user_levels/edit/\\d+|/user_levels/save/\\d+',0,0,0),(30,3,'Delete user level',1,1,'/user_levels/delete/\\d+',0,0,0),(31,3,'Watch records',1,1,'/watch_records',0,1,0),(32,3,'Delete watch record',1,1,'/watch_records/delete/\\d+',0,0,0),(33,2,'Ads',1,1,'/ads',0,1,0),(34,2,'Add/edit ads',1,1,'/ads/edit/\\d+|/ads/save/\\d+',0,0,0),(35,2,'Delete ads',1,1,'/ads/delete/\\d+',0,0,0),(36,0,'System Management',0,1,'#',0,1,0),(37,36,'Edit config',1,1,'/configs/edit/1',0,1,0),(38,36,'Save config',1,1,'/configs/save/1',0,0,0),(39,36,'Navs',1,1,'/navs',0,1,0),(40,36,'Add/edit nav',1,1,'/navs/edit/\\d+|/navs/save/\\d+',0,0,0),(41,36,'Delete nav',1,1,'/navs/delete/\\d+',0,0,0),(42,2,'Video authors',1,1,'/video_authors',0,1,0),(43,2,'Add/edit video author',1,1,'/video_authors/edit/\\d+|/video_authors/save/\\d+',0,0,0),(44,2,'Delete video author',1,1,'/video_authors/delete/\\d+',0,0,0);
/*!40000 ALTER TABLE `menus` ENABLE KEYS */;
UNLOCK TABLES;

--
-- Table structure for table `navs`
--

DROP TABLE IF EXISTS `navs`;
/*!40101 SET @saved_cs_client     = @@character_set_client */;
/*!40101 SET character_set_client = utf8 */;
CREATE TABLE `navs` (
  `id` int(10) unsigned NOT NULL AUTO_INCREMENT,
  `name` varchar(20) NOT NULL DEFAULT '',
  `url` varchar(200) NOT NULL DEFAULT '',
  `is_blank` tinyint(3) unsigned NOT NULL DEFAULT '0',
  `remark` varchar(100) NOT NULL DEFAULT '',
  `seq` int(11) NOT NULL DEFAULT '0',
  PRIMARY KEY (`id`)
) ENGINE=InnoDB AUTO_INCREMENT=6 DEFAULT CHARSET=utf8;
/*!40101 SET character_set_client = @saved_cs_client */;

--
-- Dumping data for table `navs`
--

LOCK TABLES `navs` WRITE;
/*!40000 ALTER TABLE `navs` DISABLE KEYS */;
INSERT INTO `navs` VALUES (1,'Home page','/',0,'',9999),(2,'All videos','/videos',0,'',988),(3,'About Us','/about',0,'',800),(4,'Client messages','/feedback',0,'',700),(5,'Contact us','/contact',0,'',600);
/*!40000 ALTER TABLE `navs` ENABLE KEYS */;
UNLOCK TABLES;

--
-- Table structure for table `user_levels`
--

DROP TABLE IF EXISTS `user_levels`;
/*!40101 SET @saved_cs_client     = @@character_set_client */;
/*!40101 SET character_set_client = utf8 */;
CREATE TABLE `user_levels` (
  `id` int(10) unsigned NOT NULL AUTO_INCREMENT,
  `name` varchar(20) NOT NULL DEFAULT '',
  `remark` varchar(100) NOT NULL DEFAULT '',
  `watch_per_day` int(10) unsigned NOT NULL DEFAULT '0',
  `score_min` int(10) unsigned NOT NULL DEFAULT '0',
  `score_max` int(10) unsigned NOT NULL DEFAULT '0',
  `seq` int(11) NOT NULL DEFAULT '0',
  PRIMARY KEY (`id`)
) ENGINE=InnoDB AUTO_INCREMENT=6 DEFAULT CHARSET=utf8;
/*!40101 SET character_set_client = @saved_cs_client */;

--
-- Dumping data for table `user_levels`
--

LOCK TABLES `user_levels` WRITE;
/*!40000 ALTER TABLE `user_levels` DISABLE KEYS */;
INSERT INTO `user_levels` VALUES (1,'VIP','Basic level VIP',0,0,0,0),(2,'Bronze VIP','Bronze VIP',0,0,0,0),(3,'Silver VIP','Silver VIP',0,0,0,0),(4,'Gold VIP','Gold VIP',0,0,0,0),(5,'Diamond VIP','Diamond VIP',0,0,0,0);
/*!40000 ALTER TABLE `user_levels` ENABLE KEYS */;
UNLOCK TABLES;

--
-- Table structure for table `users`
--

DROP TABLE IF EXISTS `users`;
/*!40101 SET @saved_cs_client     = @@character_set_client */;
/*!40101 SET character_set_client = utf8 */;
CREATE TABLE `users` (
  `id` int(10) unsigned NOT NULL AUTO_INCREMENT,
  `name` varchar(20) NOT NULL DEFAULT '',
  `password` char(32) NOT NULL DEFAULT '',
  `secret` char(32) NOT NULL DEFAULT '',
  `mail` varchar(100) NOT NULL DEFAULT '',
  `last_ip` varchar(32) NOT NULL DEFAULT '',
  `state` tinyint(3) unsigned NOT NULL DEFAULT '0',
  `login_count` int(10) unsigned NOT NULL DEFAULT '0',
  `level_id` int(10) unsigned NOT NULL DEFAULT '0',
  `score` int(10) unsigned NOT NULL DEFAULT '0',
  `last_login` int(10) unsigned NOT NULL DEFAULT '0',
  `created` int(10) unsigned NOT NULL DEFAULT '0',
  `updated` int(10) unsigned NOT NULL DEFAULT '0',
  `remark` varchar(200) NOT NULL DEFAULT '',
  PRIMARY KEY (`id`),
  KEY `name` (`name`)
) ENGINE=InnoDB AUTO_INCREMENT=2 DEFAULT CHARSET=utf8;
/*!40101 SET character_set_client = @saved_cs_client */;

--
-- Dumping data for table `users`
--

LOCK TABLES `users` WRITE;
/*!40000 ALTER TABLE `users` DISABLE KEYS */;
INSERT INTO `users` VALUES (1,'user','200820e3227815ed1756a6b531e7e0d2','','user@gmail.com','',0,0,0,0,0,1580470174,1580470174,'');
/*!40000 ALTER TABLE `users` ENABLE KEYS */;
UNLOCK TABLES;

--
-- Table structure for table `video_authors`
--

DROP TABLE IF EXISTS `video_authors`;
/*!40101 SET @saved_cs_client     = @@character_set_client */;
/*!40101 SET character_set_client = utf8 */;
CREATE TABLE `video_authors` (
  `id` int(10) unsigned NOT NULL AUTO_INCREMENT,
  `name` varchar(20) NOT NULL DEFAULT '',
  `remark` varchar(500) DEFAULT '',
  `seq` int(11) NOT NULL DEFAULT '0',
  PRIMARY KEY (`id`)
) ENGINE=InnoDB AUTO_INCREMENT=7 DEFAULT CHARSET=utf8;
/*!40101 SET character_set_client = @saved_cs_client */;

--
-- Dumping data for table `video_authors`
--

LOCK TABLES `video_authors` WRITE;
/*!40000 ALTER TABLE `video_authors` DISABLE KEYS */;
INSERT INTO `video_authors` VALUES (1,'default','default',0),(2,'Commander Tang said movies','Station B: https://space.bilibili.com/98605231?from=search&seid=7447429227219220401\r\nIQIYI: https://www.iqiyi.com/u/2322942463/videos',0),(3,'Science fiction dream workshop','Tell every science fiction story with your heart!\r\nStation B: https://space.bilibili.com/108425972/',0),(4,'Yue brother talks about movies','Brother Yu only made good -looking movies, retained the most essence, and talked with humorous and humorous commentary, and dedicated to everyone with a joke and attitude.',0),(5,'Datong watching movies','From writing, to editing, to publishing, it is completed by Dacong alone.Do not pursue the amount of running, only grind the boutique.Your support is Dacong best original motivation!',0),(6,'Did you watch a movie?','Good movies, change your cognition. Dagged video, please subscribe to the channel: Make up to watch a movie.',0);
/*!40000 ALTER TABLE `video_authors` ENABLE KEYS */;
UNLOCK TABLES;

--
-- Table structure for table `video_categories`
--

DROP TABLE IF EXISTS `video_categories`;
/*!40101 SET @saved_cs_client     = @@character_set_client */;
/*!40101 SET character_set_client = utf8 */;
CREATE TABLE `video_categories` (
  `id` int(10) unsigned NOT NULL AUTO_INCREMENT,
  `name` varchar(20) NOT NULL DEFAULT '',
  `remark` varchar(100) NOT NULL DEFAULT '',
  `seq` int(11) NOT NULL DEFAULT '0',
  PRIMARY KEY (`id`)
) ENGINE=InnoDB AUTO_INCREMENT=3 DEFAULT CHARSET=utf8;
/*!40101 SET character_set_client = @saved_cs_client */;

--
-- Dumping data for table `video_categories`
--

LOCK TABLES `video_categories` WRITE;
/*!40000 ALTER TABLE `video_categories` DISABLE KEYS */;
INSERT INTO `video_categories` VALUES (1,'TV drama','TV drama',0),(2,'Movie','Movie',0);
/*!40000 ALTER TABLE `video_categories` ENABLE KEYS */;
UNLOCK TABLES;

--
-- Table structure for table `video_replies`
--

DROP TABLE IF EXISTS `video_replies`;
/*!40101 SET @saved_cs_client     = @@character_set_client */;
/*!40101 SET character_set_client = utf8 */;
CREATE TABLE `video_replies` (
  `id` int(10) unsigned NOT NULL AUTO_INCREMENT,
  `video_id` int(10) unsigned NOT NULL DEFAULT '0',
  `reply_id` int(10) unsigned NOT NULL DEFAULT '0',
  `user_id` int(10) unsigned NOT NULL DEFAULT '0',
  `user_name` varchar(200) NOT NULL DEFAULT '',
  `content` text,
  `seq` int(11) NOT NULL DEFAULT '0',
  `state` tinyint(3) unsigned NOT NULL DEFAULT '0',
  `created` int(10) unsigned NOT NULL DEFAULT '0',
  PRIMARY KEY (`id`)
) ENGINE=InnoDB AUTO_INCREMENT=2 DEFAULT CHARSET=utf8;
/*!40101 SET character_set_client = @saved_cs_client */;

--
-- Dumping data for table `video_replies`
--

LOCK TABLES `video_replies` WRITE;
/*!40000 ALTER TABLE `video_replies` DISABLE KEYS */;
INSERT INTO `video_replies` VALUES (1,0,0,0,'user','Yes, this wave 666',0,0,1580470174);
/*!40000 ALTER TABLE `video_replies` ENABLE KEYS */;
UNLOCK TABLES;

--
-- Table structure for table `video_tags`
--

DROP TABLE IF EXISTS `video_tags`;
/*!40101 SET @saved_cs_client     = @@character_set_client */;
/*!40101 SET character_set_client = utf8 */;
CREATE TABLE `video_tags` (
  `id` int(10) unsigned NOT NULL AUTO_INCREMENT,
  `name` varchar(20) NOT NULL DEFAULT '',
  `remark` varchar(100) NOT NULL DEFAULT '',
  `seq` int(11) NOT NULL DEFAULT '0',
  PRIMARY KEY (`id`)
) ENGINE=InnoDB AUTO_INCREMENT=63 DEFAULT CHARSET=utf8;
/*!40101 SET character_set_client = @saved_cs_client */;

--
-- Dumping data for table `video_tags`
--

LOCK TABLES `video_tags` WRITE;
/*!40000 ALTER TABLE `video_tags` DISABLE KEYS */;
INSERT INTO `video_tags` VALUES (1,'Plot','',9990),(2,'comedy','',9980),(3,'action','',9970),(4,'love','',9960),(5,'Science fiction','',9950),(6,'Suspense','',9940),(7,'Thrilling','',9930),(8,'fear','',9920),(9,'crime','',9910),(10,'homosexual','',9890),(11,'music','',9880),(12,'Song and dance','',9870),(13,'biography','',9860),(14,'history','',9840),(15,'war','',9830),(16,'west','',9820),(17,'Fantasy','',9810),(18,'adventure','',9790),(19,'disaster','',9780),(20,'Martial arts','',9770),(21,'Eroticism','',9760),(22,'Chinese mainland','',8990),(23,'U.S.','',8980),(24,'Hongkong','',8970),(25,'Taiwan','',8960),(26,'Japan','',8950),(27,'South Korea','',8940),(28,'U.K.','',8930),(29,'France','',8920),(30,'Germany','',8910),(31,'Italy','',8900),(32,'Spain','',8890),(33,'India','',8880),(34,'Thailand','',8870),(35,'Russia','',8860),(36,'Canada','',8850),(37,'Australia','',8840),(38,'Sweden','',8830),(39,'Brazil','',8820),(40,'Denmark','',8810),(41,'other','',8800),(42,'2019','',7990),(43,'2018','',7980),(44,'2017','',7970),(45,'2016','',7960),(46,'2015','',7950),(47,'2014','',7940),(48,'2013','',7930),(49,'2012','',7920),(50,'2011','',7910),(51,'2010','',7900),(52,'2009','',7890),(53,'2008','',7880),(54,'2007','',7870),(55,'2006','',7860),(56,'2005','',7850),(57,'2004','',7840),(58,'2003','',7830),(59,'2002','',7820),(60,'2001','',7810),(61,'2000','',7800),(62,'Earlier','',7790);
/*!40000 ALTER TABLE `video_tags` ENABLE KEYS */;
UNLOCK TABLES;

--
-- Table structure for table `videos`
--

DROP TABLE IF EXISTS `videos`;
/*!40101 SET @saved_cs_client     = @@character_set_client */;
/*!40101 SET character_set_client = utf8 */;
CREATE TABLE `videos` (
  `id` int(10) unsigned NOT NULL AUTO_INCREMENT,
  `title` varchar(50) NOT NULL DEFAULT '',
  `remark` varchar(100) NOT NULL DEFAULT '',
  `cover_image` varchar(200) NOT NULL DEFAULT '',
  `duration` int(10) unsigned NOT NULL DEFAULT '0',
  `seq` int(11) NOT NULL DEFAULT '0',
  `state` tinyint(3) unsigned NOT NULL DEFAULT '0',
  `created` int(10) unsigned NOT NULL DEFAULT '0',
  `updated` int(10) unsigned NOT NULL DEFAULT '0',
  `content` text,
  `category_id` int(10) unsigned DEFAULT '0',
  `tag_ids` varchar(500) NOT NULL DEFAULT '',
  `author_id` int(10) unsigned DEFAULT '0',
  `url` varchar(200) NOT NULL DEFAULT '',
  `is_recommended` tinyint(4) NOT NULL DEFAULT '0',
  PRIMARY KEY (`id`)
) ENGINE=InnoDB AUTO_INCREMENT=13 DEFAULT CHARSET=utf8;
/*!40101 SET character_set_client = @saved_cs_client */;

--
-- Dumping data for table `videos`
--

LOCK TABLES `videos` WRITE;
/*!40000 ALTER TABLE `videos` DISABLE KEYS */;
INSERT INTO `videos` VALUES (1,'Battle of Deep Sea','Scientists discovered a kind of combustible fish, and they began to transform, resulting in horrible monsters!','/upload/2020/02/05/YpbyOTgJHeYP836D.png',210,0,1,1581335253,1581335253,'The slum guy counterattack led the army to break through the world number one game company to marry Bai Fumei',2,'1,10,7',2,'https://www.youtube.com/watch?v=C5IAlShJhhg',1),(2,'influenza','A small cold is about to die hundreds of thousands of people in South Korea','/upload/2020/02/05/YS9XXOcBiToBG5Fp.png',0,0,1,1581335233,1581335233,'asdfasdf',2,'1,19,27,7',2,'https://www.youtube.com/watch?v=aLsplmkS0Vo',1),(3,'Raptor Team: Great Jlowsome Liberation','Deep egg analysis!The old lady is cool!The DC universe will pull back another game!','/upload/2020/02/10/bks5628jC2bRVUWQ.png',0,0,1,1581335259,1581335259,NULL,2,'1,10,11',5,'https://www.youtube.com/watch?v=4s9hJSloiP4',1),(4,'Zhu Di','2020 Oscar Movie: How difficult is it to find someone who loves me?','/upload/2020/02/10/J7zZKxBEKSv32xPI.png',0,0,1,1581335264,1581335264,NULL,2,'1,10,26',5,'https://www.youtube.com/watch?v=-oPGx4XNqzo',1),(5,'Love to have a life','After death, I waited for you for 50 years, Yu Feihong 10 -year debut work','/upload/2020/02/10/Nxlqt4u1rlxh17Wi.png',0,0,1,1581335270,1581335270,NULL,2,'1,10',6,'https://www.youtube.com/watch?v=q9jUkDBLY2w',1),(6,'Label','In the massage room of the blind man, there is nowhere to place feelings and desires','/upload/2020/02/10/gtqqYecbHRwV0j5U.png',0,0,1,1581335275,1581335275,NULL,2,'1,10,11,13',6,'https://www.youtube.com/watch?v=H90E2mP5W5E',1),(7,'Still Alice','The days after the diagnosis, chronic death every day','/upload/2020/02/10/lJGIZW5NkuW0dmbW.png',0,0,1,1581335280,1581335280,NULL,2,'10,11,14,15,18,21',6,'https://www.youtube.com/watch?v=_tQHXp9H-L0',1),(8,'wave','A horrible human experiment, real adaptation','/upload/2020/02/10/5zpnVyd5g4axS02f.png',0,0,1,1581335286,1581335286,NULL,2,'1,10,11,12,2',6,'https://www.youtube.com/watch?v=vKpLJx1cpds',1),(9,'Father banner','During World War II, it became a Pacific meat grinding machine','/upload/2020/02/10/KjsmhNBD78fUjWhH.png',0,0,1,1581335291,1581335291,NULL,2,'1,10,11,12,15,16',6,'https://www.youtube.com/watch?v=P3qxq0ebH8c',1),(10,'August: Osirizhi County','What experience is there a thin mother?','/upload/2020/02/10/GNtRBxrziAhl8T7A.png',0,0,1,1581335296,1581335296,NULL,2,'1,13,17,20',6,'https://www.youtube.com/watch?v=ZwZWDewDsP4',1),(11,'The last Mossigan','How did the Indians disappear in European Black History?','/upload/2020/02/10/HbqCaFgKEpggORTb.png',0,0,1,1581334731,0,NULL,2,'13,14,17,18,25',6,'https://www.youtube.com/watch?v=69veiSnIfVk',0),(12,'Jing Yi stabbing King Qin','Why did Qin Shihuang buried Zhao Guo child alive?','/upload/2020/02/10/RFA04Q35POWEi5rX.png',0,0,0,1581334791,0,NULL,2,'17,20,24,28',6,'https://www.youtube.com/watch?v=Ovp3zw7PHJU',0);
/*!40000 ALTER TABLE `videos` ENABLE KEYS */;
UNLOCK TABLES;

--
-- Table structure for table `watch_records`
--

DROP TABLE IF EXISTS `watch_records`;
/*!40101 SET @saved_cs_client     = @@character_set_client */;
/*!40101 SET character_set_client = utf8 */;
CREATE TABLE `watch_records` (
  `id` int(10) unsigned NOT NULL AUTO_INCREMENT,
  `user_id` int(10) unsigned NOT NULL DEFAULT '0',
  `user_name` varchar(20) NOT NULL DEFAULT '',
  `video_id` int(10) unsigned NOT NULL DEFAULT '0',
  `created` int(10) unsigned NOT NULL DEFAULT '0',
  PRIMARY KEY (`id`),
  KEY `user_id` (`user_id`),
  KEY `video_id` (`video_id`)
) ENGINE=InnoDB AUTO_INCREMENT=2 DEFAULT CHARSET=utf8;
/*!40101 SET character_set_client = @saved_cs_client */;

--
-- Dumping data for table `watch_records`
--

LOCK TABLES `watch_records` WRITE;
/*!40000 ALTER TABLE `watch_records` DISABLE KEYS */;
INSERT INTO `watch_records` VALUES (1,1,'user',1,1580470174);
/*!40000 ALTER TABLE `watch_records` ENABLE KEYS */;
UNLOCK TABLES;
/*!40103 SET TIME_ZONE=@OLD_TIME_ZONE */;

/*!40101 SET SQL_MODE=@OLD_SQL_MODE */;
/*!40014 SET FOREIGN_KEY_CHECKS=@OLD_FOREIGN_KEY_CHECKS */;
/*!40014 SET UNIQUE_CHECKS=@OLD_UNIQUE_CHECKS */;
/*!40101 SET CHARACTER_SET_CLIENT=@OLD_CHARACTER_SET_CLIENT */;
/*!40101 SET CHARACTER_SET_RESULTS=@OLD_CHARACTER_SET_RESULTS */;
/*!40101 SET COLLATION_CONNECTION=@OLD_COLLATION_CONNECTION */;
/*!40111 SET SQL_NOTES=@OLD_SQL_NOTES */;

-- Dump completed on 2020-02-10 20:00:14

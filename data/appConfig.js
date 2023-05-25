

const config = {
    siteTitle: 'Ravi Sankar Karuturi',
    siteUrl: 'https://www.karuturirs.com',
    userEmail: 'hello@karuturirs.com',
    userTwitter: 'karuturirs',
    linkedInUrl: 'https://www.linkedin.com/in/ravisankarkaruturi/',
   
    projects: [
        {
            id: 4,
            name: 'TimePropose.com',
            title: 'Web tool to share availabilty between timezones with ease.',
            url: 'https://github.com/Karuturirs/timepropose.git',
            tags: ["ELM"]
        },
        {
            id: 3,
            name: 'FoodTrucksInSF',
            title: 'CMD tool to find foodtruck in SFO streets.',
            url: 'https://github.com/Karuturirs/FoodTruckInSF',
            tags: ["java"]
        },
        {
            id: 2,
            name: 'mocktail.bird',
            title: 'Mocking application for day to day development use',
            url: 'https://github.com/mocktaillbird/mocktail.bird',
            tags: ["java","mongodb","docker"]
        },
        {
            id: 1,
            name: 'karuturirs.com',
            title: 'Personal website build for my timeline ',
            url: "#",
            tags: ["gatsby","graphql","node"]
        }
    ],
    achievements: [
        {
            id: 1,
            title: 'Machine learning Certification',
            subTitle: 'Issued: June 2022', 
            more : 'Issued by: eCornell'
        },
        {
            id: 2,
            title: 'Digital Marketing',
            subTitle: 'Issued: Dec 2022', 
            more : 'Issued by: Thinkful'
        },
        {
            id: 1,
            title: 'Apache Cassandra 3.x Developer Associate Certification',
            subTitle: 'Issued: July 2020', 
            more : 'Credential Id: 7212832b-6284-4c52-8d3f-b807d829fd14'
        }
    ],
}


module.exports = config